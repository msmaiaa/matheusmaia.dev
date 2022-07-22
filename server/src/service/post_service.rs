use crate::common_types::{AppError, Pageable, Post, PostFilters};
use crate::config::Context;

pub struct PostService;
use crate::dto::CreatePostPayload;
use crate::repository::post_repository;
impl PostService {
    pub async fn create_post(
        ctx: &Context,
        data: &CreatePostPayload,
        user_id: &i32,
    ) -> Result<(), AppError> {
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());

        if let Some(tags) = &data.tags {
            repo.create_post_with_tags(data, user_id, &slug::slugify(data.title.clone()), &tags)
                .await
                .map(|_| ())
                .map_err(|err| {
                    println!("Error on post_repo/create_post {:?}", err.to_string());
                    AppError::Unknown
                })
        } else {
            repo.create_post(data, user_id, &slug::slugify(data.title.clone()))
                .await
                .map(|_| ())
                .map_err(|err| {
                    println!("Error on post_repo/create_post {:?}", err.to_string());
                    AppError::Unknown
                })
        }
    }

    pub async fn delete_post(ctx: &Context, id: &i32) -> Result<(), AppError> {
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());
        repo.delete_post(id).await.map(|_| ()).map_err(|err| {
            println!("Error on post_repo/delete_post {:?}", err.to_string());
            AppError::Unknown
        })
    }

    pub async fn update_post(ctx: &Context, data: &Post, user_id: &i32) -> Result<(), AppError> {
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
    ) -> Result<(Vec<Post>, i64), AppError> {
        let repo = post_repository::PostRepository::new(ctx.db_pool.clone());
        repo.find_many(pagination, query)
            .await
            .map(|rows| {
                let parsed: Vec<Post> = rows.into_iter().map(Post::from).collect();
                let total = parsed
                    .last()
                    .map(|last| last.totalrows.unwrap_or(0))
                    .unwrap_or(0);
                (parsed, total)
            })
            .map_err(|err| {
                println!("error on user post_repo/find_many: {:?}", err);
                AppError::Unknown
            })
    }
}
