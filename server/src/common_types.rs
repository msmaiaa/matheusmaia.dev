use chrono::Utc;

use poem_openapi::{payload::Json, ApiResponse, Object};
use serde::{self, Deserialize};

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
    pub id: u32,
    pub name: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct TagFilters {
    pub name: Option<String>,
}

#[derive(Object)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub admin: i8,
    pub createdAt: chrono::DateTime<Utc>,
    pub updatedAt: chrono::DateTime<Utc>,
}

#[derive(Object, Clone, Deserialize, sqlx::FromRow)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub slug: String,
    pub content: String,
    pub published: i8,
    pub authorId: u32,
    pub createdAt: chrono::DateTime<Utc>,
    pub updatedAt: chrono::DateTime<Utc>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct PostFilters {
    pub title: Option<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Pageable {
    pub skip: Option<i64>,
    pub take: Option<i64>,
}
