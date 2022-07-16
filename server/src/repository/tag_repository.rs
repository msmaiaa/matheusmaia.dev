use std::sync::Arc;

use sqlx::mysql::MySqlQueryResult;

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

    pub async fn create(&self, name: &str) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!("INSERT INTO Tag (name) values (?)", name)
            .execute(&*self.db_pool)
            .await
    }

    pub async fn delete(&self, id: &u64) -> Result<Option<()>, sqlx::Error> {
        sqlx::query!("DELETE FROM Tag WHERE id=?", id)
            .execute(&*self.db_pool)
            .await
            .map(|res| match res.rows_affected() > 0 {
                true => Some(()),
                false => None,
            })
    }

    pub async fn find_many(
        &self,
        pagination: Pageable,
        query: TagFilters,
    ) -> Result<Vec<Tag>, sqlx::Error> {
        //	TODO: add pagination
        sqlx::query_as!(Tag, "SELECT * FROM Tag")
            .fetch_all(&*self.db_pool)
            .await
        // if let Some(skip) = pagination.skip {
        //     query = query.skip(skip);
        // }
        // if let Some(take) = pagination.take {
        //     query = query.take(take);
        // }
        // query.exec().await
    }
}
