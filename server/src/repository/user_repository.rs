use std::sync::Arc;

use crate::{common_types::User, database::DbPool};
pub struct UserRepository {
    db_client: Arc<DbPool>,
}

impl UserRepository {
    pub fn new(db_client: Arc<DbPool>) -> UserRepository {
        UserRepository { db_client }
    }

    pub async fn find_by_username(&self, username: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM Users WHERE username=$1", username)
            .fetch_optional(&*self.db_client)
            .await
            .unwrap_or_else(|err| {
                println!("Error on find_by_username on user_repository: {:?}", err);
                None
            })
    }

    pub async fn create(&self, username: &str, password: &str, admin: bool) -> Option<i32> {
        sqlx::query!(
            "INSERT INTO Users (username, password, admin) VALUES ($1, $2, $3) RETURNING id",
            username,
            password,
            admin
        )
        .fetch_one(&*self.db_client)
        .await
        .map_or_else(
            |err| {
                println!(
                    "error on user_repository/create{:?}",
                    err.as_database_error()
                );
                None
            },
            |res| Some(res.id),
        )
    }
}
