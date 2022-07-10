<<<<<<< HEAD
=======
use poem::http::Error;
>>>>>>> 2e519a5 (refactor: better error handling)
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
