use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{self, Deserialize, Serialize};
use sqlx::{postgres::PgRow, Row};

#[derive(Object)]
pub struct ErrorMessage {
    pub message: String,
}

impl ErrorMessage {
    pub fn as_json(message: String) -> Json<ErrorMessage> {
        Json(ErrorMessage { message: message })
    }
}
#[derive(ApiResponse)]
pub enum ResponseError {
    #[oai(status = 400)]
    BadRequest(Json<ErrorMessage>),
    #[oai(status = 500)]
    InternalServerError(Json<ErrorMessage>),
    #[oai(status = 401)]
    Unauthorized,
    #[oai(status = 404)]
    NotFound(Json<ErrorMessage>),
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Unknown,
    NotFound(String),
}

impl From<AppError> for ResponseError {
    fn from(e: AppError) -> Self {
        match e {
            AppError::BadRequest(message) => {
                ResponseError::BadRequest(ErrorMessage::as_json(message))
            }
            AppError::NotFound(message) => ResponseError::NotFound(ErrorMessage::as_json(message)),
            _ => ResponseError::InternalServerError(ErrorMessage::as_json(
                "Unknown error".to_string(),
            )),
        }
    }
}

#[derive(Object)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}

impl From<PgRow> for Tag {
    fn from(row: PgRow) -> Self {
        Tag {
            name: row.get("name"),
            id: row.get("id"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct TagFilters {
    pub name: Option<String>,
}

#[derive(Object)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
    pub password: String,
    pub admin: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Object, Serialize, Deserialize)]
pub struct SanitizedUser {
    pub id: i32,
    pub username: String,
    pub avatar_url: Option<String>,
}

impl From<User> for SanitizedUser {
    fn from(user: User) -> Self {
        SanitizedUser {
            id: user.id,
            username: user.username,
            avatar_url: user.avatar_url,
        }
    }
}

#[derive(Object, Clone, Deserialize, Serialize, sqlx::FromRow, Debug)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub author_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[oai(skip)]
    pub totalrows: Option<i64>,
}

#[derive(Object, Serialize, Deserialize)]
pub struct PostWithAuthor {
    pub id: i32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: bool,
    pub author_id: i32,
    pub author: SanitizedUser,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    #[oai(skip)]
    pub totalrows: Option<i64>,
}

impl From<PgRow> for PostWithAuthor {
    fn from(row: PgRow) -> Self {
        PostWithAuthor {
            id: row.get("id"),
            title: row.get("title"),
            slug: row.get("slug"),
            content: row.get("content"),
            published: row.get("published"),
            author_id: row.get("author_id"),
            author: SanitizedUser {
                id: row.get("author_id"),
                username: row.get("author_username"),
                avatar_url: row.get("author_avatar_url"),
            },
            totalrows: row.get("totalrows"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<PgRow> for Post {
    fn from(row: PgRow) -> Self {
        Post {
            title: row.get("title"),
            id: row.get("id"),
            slug: row.get("content"),
            published: row.get("published"),
            author_id: row.get("author_id"),
            content: row.get("content"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            totalrows: row.get("totalrows"),
        }
    }
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct PostFilters {
    pub title: Option<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Pageable {
    pub page: Option<i64>,
    pub take: Option<i64>,
}
