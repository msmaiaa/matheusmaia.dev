use sqlx::mysql::MySqlQueryResult;

use crate::{
    common_types::{CreatePostPayload, Pageable, Post, PostFilters},
    database::DbPool,
};
use std::sync::Arc;

pub struct PostRepository {
    db_pool: Arc<DbPool>,
}

impl PostRepository {
    pub fn new(db_pool: Arc<DbPool>) -> Self {
        PostRepository { db_pool }
    }
    pub async fn create_post(
        &self,
        data: &CreatePostPayload,
        user_id: &u32,
        slug: &str,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        // let mut _params: Vec<post::SetParam> =
        //     vec![post::published::set(data.published.unwrap_or(false))];
        // if let Some(tags) = &data.tags {
        //     _params.push(post::tags::link(
        //         tags.into_iter().map(|tag| tag::id::equals(*tag)).collect(),
        //     ))
        // }
        sqlx::query!(
            "INSERT INTO Post (
					content,
					title,
					slug,
					published,
					authorId
				) VALUES (?, ?, ?, ?, ?)",
            data.content,
            data.title,
            slug,
            data.published,
            user_id
        )
        .execute(&*self.db_pool)
        .await
    }

    pub async fn update_post(
        &self,
        data: &Post,
        user_id: &u32,
    ) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE Post SET content=?, title=?, slug=?, published=? WHERE authorId=?",
            data.content,
            data.title,
            data.slug,
            data.published,
            user_id
        )
        .execute(&*self.db_pool)
        .await
    }

    pub async fn delete_post(&self, id: &u32) -> Result<MySqlQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM Post WHERE id=?", id)
            .execute(&*self.db_pool)
            .await
    }

    pub async fn find_many(
        &self,
        pagination: &Pageable,
        filters: &PostFilters,
    ) -> Result<Vec<Post>, sqlx::Error> {
        sqlx::query_as!(Post, "SELECT * FROM Post")
            .fetch_all(&*self.db_pool)
            .await
        // if let Some(skip) = pagination.skip {
        //     query = query.skip(skip);
        // }
        // if let Some(take) = pagination.take {
        //     query = query.take(take);
        // }
    }
}
