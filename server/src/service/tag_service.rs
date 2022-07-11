use crate::{
    common_types::{AppError, Pageable, Tag, TagFilters},
    prisma::PrismaClient,
};

pub struct TagService;

impl TagService {
    pub async fn create_tag(
        prisma: std::sync::Arc<PrismaClient>,
        name: &str,
    ) -> Result<Tag, AppError> {
        let repo = crate::repository::tag_repository::TagRepository::new(prisma);
        repo.create(name).await.map(Tag::from).map_err(|err| {
            if err.to_string().contains("Tag_name_key") {
                return AppError::BadRequest("Tag already exists".to_string());
            }
            AppError::Unknown
        })
    }

    pub async fn delete(prisma: std::sync::Arc<PrismaClient>, id: &i32) -> Result<(), AppError> {
        let repo = crate::repository::tag_repository::TagRepository::new(prisma);
        repo.delete(id).await.map_or(Ok(()), |res| match res {
            Some(_) => Ok(()),
            None => Err(AppError::NotFound("Tag not found".to_string())),
        })
    }

    //	TODO: add query fields to swagger
    pub async fn find_many(
        prisma: std::sync::Arc<PrismaClient>,
        params: Pageable,
        query: TagFilters,
    ) -> Result<Vec<Tag>, AppError> {
        let repo = crate::repository::tag_repository::TagRepository::new(prisma);

        repo.find_many(params, query)
            .await
            .map(|val| val.into_iter().map(Tag::from).collect())
            .map_err(|err| {
                println!("{:?}", err);
                AppError::Unknown
            })
    }
}
