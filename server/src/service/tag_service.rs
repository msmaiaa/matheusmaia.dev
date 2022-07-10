use crate::{
    common_types::{AppError, Tag, TagPaginationParams},
    prisma::PrismaClient,
};

pub struct TagService;

impl TagService {
    pub async fn create_tag(
        prisma: std::sync::Arc<PrismaClient>,
        name: &str,
    ) -> Result<Tag, AppError> {
        let repo = crate::repository::tag_repository::TagRepository::new(prisma);
        match repo.create(name).await {
            Ok(tag) => Ok(Tag {
                id: tag.id,
                name: tag.name,
            }),
            Err(err) => {
                if err.to_string().contains("Tag_name_key") {
                    return Err(AppError::BadRequest("Tag already exists".to_string()));
                }
                Err(AppError::Unknown)
            }
        }
    }
    pub async fn find_many(
        prisma: std::sync::Arc<PrismaClient>,
        params: TagPaginationParams,
    ) -> Result<Vec<Tag>, AppError> {
        let repo = crate::repository::tag_repository::TagRepository::new(prisma);

        // can this type convertion be avoided?
        match repo.find_many(params).await {
            Ok(tags) => Ok(tags
                .into_iter()
                .map(|tag| Tag {
                    id: tag.id,
                    name: tag.name,
                })
                .collect::<Vec<Tag>>()),
            Err(_) => Err(AppError::Unknown),
        }
    }
}
