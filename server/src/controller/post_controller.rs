use poem::{web::Data, Request};
use poem_openapi::{payload::Json, ApiResponse, OpenApi};

use crate::{
    common_types::{CreatePostPayload, ErrorMessage, Post, ResponseError},
    config::context::Context,
    jwt::JWTAuthorization,
};

pub struct PostController;

#[derive(ApiResponse)]
enum CreatePostResponse {
    #[oai(status = 201)]
    Created(Json<Post>),
}

#[derive(ApiResponse)]
enum DeletePostResponse {
    #[oai(status = 204)]
    Deleted,
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
        //	TODO: handle empty array of tags
        if body.0.title.is_empty() || body.0.content.is_empty() {
            return Err(ResponseError::BadRequest(ErrorMessage::as_json(
                "Title and content are required".to_string(),
            )));
        }
        if let Some(tags) = &body.0.tags {
            if tags.is_empty() {
                return Err(ResponseError::BadRequest(ErrorMessage::as_json(
                    "You must have atleast one tag in the array of tags".to_string(),
                )));
            }
        }
        crate::service::PostService::create_post(data.prisma.to_owned(), &body, &auth.0.id)
            .await
            .map(|post| CreatePostResponse::Created(Json(post)))
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
        crate::service::PostService::delete_post(data.prisma.to_owned(), &id)
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
        crate::service::PostService::update_post(data.prisma.to_owned(), &body)
            .await
            .map(|_| PublishPostResponse::Ok)
            .map_err(ResponseError::from)
    }
}
