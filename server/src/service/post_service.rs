use crate::common_types::{AppError, Pageable, Post, PostFilters};
use crate::{common_types::CreatePostPayload, prisma::PrismaClient};
use prisma_client_rust::prisma_errors::{query_engine::*, UserFacingError};

pub struct PostService;
use crate::repository::post_repository;

fn is_err<T: UserFacingError>(err: &prisma_client_rust::prisma_errors::Error) -> bool {
    prisma_client_rust::error_is_type::<T>(err)
}

fn get_error(err: &prisma_client_rust::prisma_errors::Error) -> crate::common_types::AppError {
    if is_err::<RecordRequiredButNotFound>(&err) {
        return AppError::BadRequest("Record required but not found".to_string());
    } else if is_err::<UniqueKeyViolation>(&err) {
        return AppError::BadRequest("Unique key violation".to_string());
    }
    AppError::BadRequest(err.message().to_string())
}

impl PostService {
    pub async fn create_post(
        prisma: std::sync::Arc<PrismaClient>,
        data: &CreatePostPayload,
        user_id: &i32,
    ) -> Result<Post, AppError> {
        let repo = post_repository::PostRepository::new(prisma);
        let res = repo.create_post(data, user_id).await;
        if let Ok(data) = res {
            return Ok(Post::from(data));
        }
        if let Err(err) = res {
            match err {
                prisma_client_rust::Error::Execute(err) => return Err(get_error(&err)),
                _ => return Err(AppError::Unknown),
            }
        }
        Err(AppError::Unknown)
    }

    pub async fn delete_post(
        prisma: std::sync::Arc<PrismaClient>,
        id: &i32,
    ) -> Result<(), AppError> {
        let repo = post_repository::PostRepository::new(prisma);
        match repo.delete_post(id).await {
            Ok(data) => {
                if let Some(_) = data {
                    return Ok(());
                }
                return Err(AppError::BadRequest(
                    "Record required but not found".to_string(),
                ));
            }
            Err(err) => match err {
                prisma_client_rust::Error::Execute(err) => return Err(get_error(&err)),
                _ => return Err(AppError::Unknown),
            },
        }
    }

    pub async fn update_post(
        prisma: std::sync::Arc<PrismaClient>,
        data: &Post,
    ) -> Result<Post, AppError> {
        //	TODO: check if the user trying to update is the author
        let repo = post_repository::PostRepository::new(prisma);
        match repo.update_post(data).await {
            Ok(data) => {
                if let Some(data) = data {
                    return Ok(Post::from(data));
                }
                return Err(AppError::BadRequest(
                    "Record required but not found".to_string(),
                ));
            }
            Err(err) => match err {
                prisma_client_rust::Error::Execute(err) => return Err(get_error(&err)),
                _ => return Err(AppError::Unknown),
            },
        }
    }

    pub async fn find_many(
        prisma: std::sync::Arc<PrismaClient>,
        query: &PostFilters,
        pagination: &Pageable,
    ) -> Result<Vec<Post>, AppError> {
        let repo = post_repository::PostRepository::new(prisma);
        repo.find_many(pagination, query)
            .await
            .map(|vec| vec.into_iter().map(Post::from).collect())
            .map_err(|err| {
                println!("error on user find_many service: {:?}", err);
                AppError::Unknown
            })
    }
}
