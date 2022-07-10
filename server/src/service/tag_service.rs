use std::io::{Error, ErrorKind};

use crate::prisma::PrismaClient;

pub struct TagService;

pub struct Tag {
    pub id: i32,
    pub name: String,
}
impl TagService {
    pub async fn create_tag(
        prisma: std::sync::Arc<PrismaClient>,
        name: &str,
    ) -> Result<Tag, Error> {
        let repo = crate::repository::tag_repository::TagRepository::new(prisma);
        match repo.create(name).await {
            Ok(tag) => Ok(Tag {
                id: tag.id,
                name: tag.name,
            }),
            Err(err) => {
                if err.to_string().contains("Tag_name_key") {
                    return Err(Error::new(ErrorKind::AlreadyExists, "Tag already exists"));
                }
                Err(Error::new(ErrorKind::Other, err.to_string()))
            }
        }
    }
}
