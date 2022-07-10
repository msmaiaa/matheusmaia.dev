use std::sync::Arc;

use crate::{
    common_types::TagPaginationParams,
    prisma::{self, tag, PrismaClient},
};

pub struct TagRepository {
    client: Arc<PrismaClient>,
}

impl TagRepository {
    pub fn new(client: Arc<PrismaClient>) -> TagRepository {
        TagRepository { client }
    }

    pub async fn create(&self, name: &str) -> Result<prisma::tag::Data, prisma_client_rust::Error> {
        self.client
            .tag()
            .create(tag::name::set(name.to_string()), vec![])
            .exec()
            .await
    }

    pub async fn find_many(
        &self,
        pagination: TagPaginationParams,
    ) -> Result<Vec<prisma::tag::Data>, prisma_client_rust::Error> {
        let mut query = self
            .client
            .tag()
            .find_many(vec![crate::prisma::tag::name::contains(
                pagination.name.unwrap_or("".to_string()),
            )]);
        if let Some(skip) = pagination.skip {
            query = query.skip(skip);
        }
        if let Some(take) = pagination.take {
            query = query.take(take);
        }
        query.exec().await
    }
}
