use sqlx::{mysql::MySqlQueryResult, MySql, QueryBuilder};

use crate::{
    common_types::{Pageable, Post, PostFilters},
    database::DbPool,
    dto::CreatePostPayload,
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
    ) -> Result<u64, sqlx::Error> {
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
        .map(|res| res.last_insert_id())
    }

    pub async fn create_post_with_tags(
        &self,
        post_data: &CreatePostPayload,
        user_id: &u32,
        slug: &str,
        tags: &Vec<u32>,
    ) -> Result<(), sqlx::Error> {
        //	we need to use a transaction to revert the post creation if the user send an invalid tag id
        let mut transaction = self.db_pool.begin().await?;
        let created_post_id = sqlx::query!(
            "INSERT INTO Post (
				content,
				title,
				slug,
				published,
				authorId
			) VALUES (?, ?, ?, ?, ?)",
            post_data.content,
            post_data.title,
            slug,
            post_data.published,
            user_id
        )
        .execute(&mut transaction)
        .await?
        .last_insert_id();

        // this is how you insert multiple values at once
        let mut query_builder: QueryBuilder<MySql> =
            QueryBuilder::new("INSERT INTO TagOnPost(postId, tagId)");
        query_builder.push_values(tags.into_iter().take(65535 / 4), |mut b, tag| {
            b.push_bind(created_post_id).push_bind(tag);
        });
        query_builder.build().execute(&mut transaction).await?;

        transaction.commit().await?;
        Ok(())
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
        //	TODO: fix pagination
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
