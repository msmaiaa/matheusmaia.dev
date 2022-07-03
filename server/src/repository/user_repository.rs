use std::sync::Arc;

use crate::prisma::{self, user, PrismaClient};

pub struct UserRepository {
    client: Arc<PrismaClient>,
}

impl UserRepository {
    pub fn new(client: Arc<PrismaClient>) -> UserRepository {
        UserRepository { client }
    }

    pub async fn find_by_username(
        &self,
        username: &str,
    ) -> Result<Option<prisma::user::Data>, prisma_client_rust::Error> {
        self.client
            .user()
            .find_first(vec![user::username::equals(username.to_string())])
            .exec()
            .await
    }

    pub async fn create(
        &self,
        username: &str,
        password: &str,
        admin: bool,
    ) -> Result<prisma::user::Data, prisma_client_rust::Error> {
        self.client
            .user()
            .create(
                user::username::set(username.to_string()),
                user::password::set(password.to_string()),
                user::admin::set(admin),
                vec![],
            )
            .exec()
            .await
    }
}
