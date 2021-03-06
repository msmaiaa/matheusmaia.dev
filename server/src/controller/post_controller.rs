use poem::{web::Data, Request};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};
use serde::{Deserialize, Serialize};

use crate::{
    common_types::{ErrorMessage, Pageable, Post, PostFilters, ResponseError},
    config::Context,
    dto::CreatePostPayload,
    jwt::JWTAuthorization,
};

pub struct PostController;

#[derive(ApiResponse)]
enum CreatePostResponse {
    #[oai(status = 201)]
    Created,
}

#[derive(ApiResponse)]
enum DeletePostResponse {
    #[oai(status = 204)]
    Deleted,
}

#[derive(Serialize, Deserialize, Object)]
struct PostFindMany {
    pub data: Vec<Post>,
    pub total: i64,
}

#[derive(ApiResponse)]
enum GetPostsResponse {
    #[oai(status = 200)]
    Ok(Json<PostFindMany>),
}

#[derive(ApiResponse)]
enum PublishPostResponse {
    #[oai(status = 200)]
    Ok,
}

#[OpenApi(prefix_path = "/post")]
impl PostController {
    #[oai(path = "/", method = "post")]
    async fn create(
        &self,
        data: Data<&Context>,
        auth: JWTAuthorization,
        body: Json<CreatePostPayload>,
    ) -> Result<CreatePostResponse, ResponseError> {
        if body.0.title.is_empty() || body.0.content.is_empty() {
            return Err(ResponseError::BadRequest(ErrorMessage::as_json(
                "Title and content are required".to_string(),
            )));
        }
        if let Some(_tags) = &body.tags {
            if _tags.is_empty() {
                return Err(ResponseError::BadRequest(ErrorMessage::as_json(
                    "You must have atleast one tag in the array of tags".to_string(),
                )));
            }
        }

        crate::service::PostService::create_post(&data, &body, &auth.0.id)
            .await
            .map(|_| CreatePostResponse::Created)
            .map_err(|e| e.into())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete(
        &self,
        data: Data<&Context>,
        _auth: JWTAuthorization,
        req: &Request,
    ) -> Result<DeletePostResponse, ResponseError> {
        //	TODO: dont use fucking expect lol
        let id = req.path_params::<i32>().expect("error on /post/:id delete");
        crate::service::PostService::delete_post(&data, &id)
            .await
            .map(|_| DeletePostResponse::Deleted)
            .map_err(ResponseError::from)
    }

    #[oai(path = "/:id", method = "put")]
    async fn update(
        &self,
        data: Data<&Context>,
        _auth: JWTAuthorization,
        req: &Request,
        mut body: Json<Post>,
    ) -> Result<PublishPostResponse, ResponseError> {
        body.id = req
            .path_params::<i32>()
            .expect("error on /post/:id publish");
        crate::service::PostService::update_post(&data, &body, &_auth.0.id)
            .await
            .map(|_| PublishPostResponse::Ok)
            .map_err(ResponseError::from)
    }

    #[oai(path = "/", method = "get")]
    async fn find_many(
        &self,
        data: Data<&Context>,
        req: &Request,
    ) -> Result<GetPostsResponse, ResponseError> {
        let name = req.params::<PostFilters>().unwrap_or_default();
        let filters = req.params::<Pageable>().unwrap_or_default();
        crate::service::PostService::find_many(&data, &name, &filters)
            .await
            .map(|(posts, total)| GetPostsResponse::Ok(Json(PostFindMany { data: posts, total })))
            .map_err(ResponseError::from)
    }
}
