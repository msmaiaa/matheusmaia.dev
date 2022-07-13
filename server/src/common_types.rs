use poem_openapi::{payload::Json, ApiResponse, Object};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenData {
    pub username: String,
    pub id: i32,
    pub iat: i64,
    pub exp: i64,
}

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

impl From<crate::prisma::tag::Data> for Tag {
    fn from(data: crate::prisma::tag::Data) -> Self {
        Tag {
            id: data.id,
            name: data.name,
        }
    }
}

#[derive(Object)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub admin: bool,
    pub posts: Option<Vec<Post>>,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

#[derive(Object)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub published: bool,
    pub author_id: i32,
    pub created_at: chrono::DateTime<chrono::FixedOffset>,
    pub updated_at: chrono::DateTime<chrono::FixedOffset>,
}

impl From<crate::prisma::post::Data> for Post {
    fn from(data: crate::prisma::post::Data) -> Self {
        Post {
            id: data.id,
            title: data.title,
            published: data.published,
            author_id: data.author_id,
            created_at: data.created_at,
            updated_at: data.updated_at,
        }
    }
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Pageable {
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct TagFilters {
    pub name: Option<String>,
}

#[derive(serde::Deserialize, Object)]
pub struct CreatePostPayload {
    pub title: String,
    pub content: String,
    pub published: Option<bool>,
    pub tags: Option<Vec<i32>>,
}
