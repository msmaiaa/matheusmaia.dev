use crate::common_types::CreatePostPayload;
use crate::common_types::{AppError, Pageable, Post, PostFilters};
use crate::config::Context;

pub struct PostService;
use crate::repository::post_repository;
impl PostService {
    pub async fn create_post(
        ctx: &Context,
        data: &CreatePostPayload,
        user_id: &u32,
    ) -> Result<(), AppError> {
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());
        repo.create_post(data, user_id, &slug::slugify(data.title.clone()))
            .await
            .map(|_| ())
            .map_err(|err| {
                println!("Error on post_repo/create_post {:?}", err.to_string());
                AppError::Unknown
            })
    }

    pub async fn delete_post(ctx: &Context, id: &u32) -> Result<(), AppError> {
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());
        repo.delete_post(id).await.map(|_| ()).map_err(|err| {
            println!("Error on post_repo/delete_post {:?}", err.to_string());
            AppError::Unknown
        })
    }

    pub async fn update_post(ctx: &Context, data: &Post, user_id: &u32) -> Result<(), AppError> {
        //	TODO: check if the user trying to update is the author
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());
        repo.update_post(data, user_id)
            .await
            .map(|_| ())
            .map_err(|err| {
                println!("Error on post_repo/update_post {:?}", err.to_string());
                return AppError::Unknown;
            })
    }

    pub async fn find_many(
        ctx: &Context,
        query: &PostFilters,
        pagination: &Pageable,
    ) -> Result<Vec<Post>, AppError> {
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());
        repo.find_many(pagination, query).await.map_err(|err| {
            println!("error on user post_repo/find_many: {:?}", err);
            AppError::Unknown
        })
    }
}
