pub struct TagController;
use poem::{web::Data, Request};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

use crate::{
    common_types::{ResponseError, Tag, TagPaginationParams},
    config::context::Context,
};

#[derive(Object)]
struct CreateTagPayload {
    name: String,
}

#[derive(ApiResponse)]
enum CreateTagResponse {
    #[oai(status = 201)]
    Ok,
}

#[derive(ApiResponse)]
enum FindTagsResponse {
    #[oai(status = 200)]
    Ok(Json<Vec<Tag>>),
}

#[OpenApi(prefix_path = "/tag")]
impl TagController {
    #[oai(path = "/", method = "post")]
    async fn create(
        &self,
        data: Data<&Context>,
        _auth: crate::jwt::JWTAuthorization,
        body: Json<CreateTagPayload>,
    ) -> Result<CreateTagResponse, ResponseError> {
        match crate::service::TagService::create_tag(data.prisma.to_owned(), &body.0.name).await {
            Ok(_) => Ok(CreateTagResponse::Ok),
            Err(e) => Err(e.into()),
        }
    }
    #[oai(path = "/", method = "get")]
    async fn find_many(
        &self,
        req: &Request,
        data: Data<&Context>,
    ) -> Result<FindTagsResponse, ResponseError> {
        let params = req
            .params::<TagPaginationParams>()
            .unwrap_or(TagPaginationParams {
                skip: None,
                take: None,
                name: None,
            });
        match crate::service::TagService::find_many(data.prisma.to_owned(), params).await {
            Ok(tags) => Ok(FindTagsResponse::Ok(Json(tags))),
            Err(e) => Err(e.into()),
        }
    }
}
