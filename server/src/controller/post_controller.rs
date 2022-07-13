use poem::web::Data;
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
}
