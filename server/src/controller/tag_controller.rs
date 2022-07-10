pub struct TagController;
use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

use crate::config::context::Context;

#[derive(Object)]
struct CreateTagPayload {
    name: String,
}

#[derive(Object)]
struct Error {
    message: String,
}

#[derive(ApiResponse)]
enum CreateTagResponse {
    #[oai(status = 201)]
    Ok,
    #[oai(status = 500)]
    InternalServerError(Json<Error>),
}

#[OpenApi(prefix_path = "/tag")]
impl TagController {
    #[oai(path = "/", method = "post")]
    async fn create(
        &self,
        data: Data<&Context>,
        _auth: crate::jwt::JWTAuthorization,
        body: Json<CreateTagPayload>,
    ) -> CreateTagResponse {
        match crate::service::TagService::create_tag(data.prisma.to_owned(), &body.0.name).await {
            Ok(_) => CreateTagResponse::Ok,
            Err(err) => CreateTagResponse::InternalServerError(Json(Error {
                message: err.to_string(),
            })),
        }
    }
}
