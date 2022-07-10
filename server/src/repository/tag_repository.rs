use std::sync::Arc;

use crate::prisma::{self, tag, PrismaClient};

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
}
