use poem_openapi::{payload::Json, ApiResponse, Object};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenData {
    pub username: String,
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
}

pub enum AppError {
    BadRequest(String),
    Unknown,
}

impl From<AppError> for ResponseError {
    fn from(e: AppError) -> Self {
        match e {
            AppError::BadRequest(message) => {
                ResponseError::BadRequest(ErrorMessage::as_json(message))
            }
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

#[derive(serde::Deserialize, Debug, Default)]
pub struct Pageable {
    pub skip: Option<i64>,
    pub take: Option<i64>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct TagFilters {
    pub name: Option<String>,
}
