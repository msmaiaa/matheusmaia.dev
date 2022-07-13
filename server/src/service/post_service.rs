use crate::common_types::{AppError, Post, Tag};
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
        //	TODO: handle unique title
        //	TODO: handle empty array of tags
        //	TODO: handle empty content
        //	TODO: handle empty title
        // 	TODO: handle non existing tag error
        let repo = post_repository::PostRepository::new(prisma);
        let res = repo.create_post(data, user_id).await;
        if let Ok(data) = res {
            //	TODO: impl From<> for Post
            return Ok(Post {
                author_id: data.author_id,
                author: None,
                created_at: data.created_at,
                id: data.id,
                published: data.published,
                tags: data.tags.map_or(None, |tags| {
                    Some(
                        tags.into_iter()
                            .map(|tag| Tag {
                                id: tag.id,
                                name: tag.name,
                            })
                            .collect(),
                    )
                }),
                title: data.title,
                updated_at: data.updated_at,
            });
        }
        if let Err(err) = res {
            match err {
                prisma_client_rust::Error::Execute(err) => return Err(get_error(&err)),
                _ => return Err(AppError::Unknown),
            }
        }
        Err(AppError::Unknown)
    }
}
