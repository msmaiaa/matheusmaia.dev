use std::sync::Arc;

use sqlx::mysql::MySqlQueryResult;

use crate::{common_types::User, database::DbPool};
pub struct UserRepository {
    db_client: Arc<DbPool>,
}

impl UserRepository {
    pub fn new(db_client: Arc<DbPool>) -> UserRepository {
        UserRepository { db_client }
    }

    pub async fn find_by_username(&self, username: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM User WHERE username=?", username)
            .fetch_optional(&*self.db_client)
            .await
            .unwrap_or_else(|err| {
                println!("Error on find_by_username on user_repository: {:?}", err);
                None
            })
    }

    pub async fn create(&self, username: &str, password: &str, admin: i8) -> Option<u64> {
        sqlx::query_as!(
            User,
            "INSERT INTO User (username, password, admin) VALUES (?, ?, ?)",
            username,
            password,
            admin
        )
        .execute(&*self.db_client)
        .await
        .map_or_else(
            |err| {
                println!(
                    "error on user_repository/create{:?}",
                    err.as_database_error()
                );
                None
            },
            |res| Some(res.last_insert_id()),
        )
    }
}
