pub struct TagController;
use poem::{
    web::{Data, Path},
    Request,
};
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

use crate::{
    common_types::{Pageable, ResponseError, Tag, TagFilters},
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

#[derive(ApiResponse)]
enum DeleteTagResponse {
    #[oai(status = 204)]
    Ok,
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
        crate::service::TagService::create_tag(data.prisma.to_owned(), &body.0.name)
            .await
            .map(|_| CreateTagResponse::Ok)
            .map_err(|e| e.into())
    }
    #[oai(path = "/", method = "get")]
    async fn find_many(
        &self,
        req: &Request,
        data: Data<&Context>,
    ) -> Result<FindTagsResponse, ResponseError> {
        let filters = req.params::<Pageable>().unwrap_or_default();
        let other_params = req.params::<TagFilters>().unwrap_or_default();
        crate::service::TagService::find_many(data.prisma.to_owned(), filters, other_params)
            .await
            .map(|tags| FindTagsResponse::Ok(Json(tags)))
            .map_err(|e| e.into())
    }

    #[oai(path = "/:id", method = "delete")]
    async fn delete(
        &self,
        req: &Request,
        data: Data<&Context>,
    ) -> Result<DeleteTagResponse, ResponseError> {
        let id = req.path_params::<i32>().expect("Error on path params");

        crate::service::TagService::delete(data.prisma.to_owned(), &id)
            .await
            .map(|_| DeleteTagResponse::Ok)
            .map_err(|e| e.into())
    }
}
