use crate::{
    common_types::{Pageable, Post, PostFilters},
    database::{build_paginated_query, DbPool},
    dto::CreatePostPayload,
};
use sqlx::{
    postgres::{PgQueryResult, PgRow},
    Postgres, QueryBuilder,
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
        user_id: &i32,
        slug: &str,
    ) -> Result<i32, sqlx::Error> {
        sqlx::query!(
            "INSERT INTO Post (
					content,
					title,
					slug,
					published,
					author_id
				) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            data.content,
            data.title,
            slug,
            data.published,
            *user_id
        )
        .fetch_one(&*self.db_pool)
        .await
        .map(|res| res.id)
    }

    pub async fn create_post_with_tags(
        &self,
        post_data: &CreatePostPayload,
        user_id: &i32,
        slug: &str,
        tags: &Vec<i32>,
    ) -> Result<(), sqlx::Error> {
        //	we need to use a transaction to revert the post creation if the user send an invalid tag id
        let mut transaction = self.db_pool.begin().await?;
        let created_post_id = sqlx::query!(
            "INSERT INTO post (
				content,
				title,
				slug,
				published,
				author_id
			) VALUES ($1, $2, $3, $4, $5) RETURNING id",
            post_data.content,
            post_data.title,
            slug,
            post_data.published,
            *user_id
        )
        .fetch_one(&mut transaction)
        .await?
        .id;

        // this is how you insert multiple values at once
        let mut query_builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO tag_on_post(post_id, tag_id)");
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
        user_id: &i32,
    ) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!(
            "UPDATE Post SET content=$1, title=$2, slug=$3, published=$4 WHERE author_id=$5",
            data.content,
            data.title,
            data.slug,
            data.published,
            *user_id
        )
        .execute(&*self.db_pool)
        .await
    }

    pub async fn delete_post(&self, id: &i32) -> Result<PgQueryResult, sqlx::Error> {
        sqlx::query!("DELETE FROM Post WHERE id=$1", id)
            .execute(&*self.db_pool)
            .await
    }

    pub async fn find_many(
        &self,
        pagination: &Pageable,
        filters: &PostFilters,
    ) -> Result<Vec<PgRow>, sqlx::Error> {
        let mut query_string =
            "SELECT id, content, title, slug, published, author_id, created_at, updated_at
			FROM post"
                .to_string();

        if let Some(title) = &filters.title {
            query_string.push_str(&format!(" WHERE title ILIKE '%{}%'", *title));
        }

        let built_query =
            build_paginated_query(&query_string, &None, &pagination.page, &pagination.take);
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(built_query);
        builder.build().fetch_all(&*self.db_pool).await
    }

    pub async fn find_many_with_author(
        &self,
        pagination: &Pageable,
        filters: &PostFilters,
    ) -> Result<Vec<PgRow>, sqlx::Error> {
        let mut query_string = r#"
						SELECT 
							p.id, 
							p.content, 
							p.title, 
							p.slug, 
							p.published, 
							p.author_id, 
							p.created_at,
							p.updated_at,
              u.username as author_username,
              u.avatar_url as author_avatar_url
						FROM post p
						LEFT JOIN users u on u.id = p.author_id
						"#
        .to_string();

        if let Some(title) = &filters.title {
            query_string.push_str(&format!(" WHERE p.title ILIKE '%{}%'", *title));
        }

        let built_query =
            build_paginated_query(&query_string, &None, &pagination.page, &pagination.take);
        let mut builder: QueryBuilder<Postgres> = QueryBuilder::new(built_query);
        builder.build().fetch_all(&*self.db_pool).await
    }
}
