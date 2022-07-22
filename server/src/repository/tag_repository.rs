use std::sync::Arc;

use sqlx::postgres::PgQueryResult;

use crate::{
    common_types::{Pageable, Tag, TagFilters},
    database::DbPool,
};

pub struct TagRepository {
    db_pool: Arc<DbPool>,
}

impl TagRepository {
    pub fn new(db_pool: Arc<DbPool>) -> TagRepository {
        TagRepository { db_pool }
    }

    pub async fn create(&self, name: &str) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("INSERT INTO Tag (name) values ($1)", name)
            .execute(&*self.db_pool)
            .await
    }

    pub async fn delete(&self, id: &i32) -> Result<Option<()>, sqlx::Error> {
        sqlx::query!("DELETE FROM Tag WHERE id=$1", id)
            .execute(&*self.db_pool)
            .await
            .map(|res| match res.rows_affected() > 0 {
                true => Some(()),
                false => None,
            })
    }

    pub async fn find_many(&self, query: &TagFilters) -> Result<Vec<Tag>, sqlx::Error> {
        let mut query_string = "SELECT * FROM Tag".to_string();
        if let Some(name) = &query.name {
            query_string.push_str(&format!(" WHERE name ILIKE '%{}%'", name));
        }
        sqlx::query(&query_string)
            .fetch_all(&*self.db_pool)
            .await
            .map(|rows| rows.into_iter().map(Tag::from).collect())
    }
}
