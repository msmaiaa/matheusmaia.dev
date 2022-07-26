use crate::repository::TagRepository;
use crate::{
    common_types::{AppError, Tag, TagFilters},
    config::Context,
};

pub struct TagService;

impl TagService {
    pub async fn create_tag(ctx: &Context, name: &str) -> Result<(), AppError> {
        let repo = TagRepository::new(ctx.db_pool.clone());
        repo.create(name).await.map(|_| ()).map_err(|err| {
            println!("Error on tag creation {:?}", err.to_string());
            AppError::BadRequest(err.to_string())
        })
    }

    pub async fn delete(ctx: &Context, id: &i32) -> Result<(), AppError> {
        let repo = TagRepository::new(ctx.db_pool.clone());
        repo.delete(id).await.map_or(Ok(()), |res| match res {
            Some(_) => Ok(()),
            None => Err(AppError::NotFound("Tag not found".to_string())),
        })
    }

    //	TODO: add query fields to swagger
    pub async fn find_many(ctx: &Context, query: &TagFilters) -> Result<Vec<Tag>, AppError> {
        let repo = TagRepository::new(ctx.db_pool.clone());

        repo.find_many(query).await.map_err(|err| {
            println!("{:?}", err);
            AppError::Unknown
        })
    }
}
