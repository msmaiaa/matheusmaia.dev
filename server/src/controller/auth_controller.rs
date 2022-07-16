use crate::{
    common_types::ResponseError, config::Context, jwt::JWTAuthorization, service::AuthService,
};
use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

pub struct AuthController;

#[derive(Object)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Object)]
struct LoginResponsePayload {
    token: String,
    username: String,
}

#[derive(ApiResponse)]
enum LoginResponse {
    #[oai(status = 200)]
    Ok(Json<LoginResponsePayload>),
}

#[derive(Object)]
struct GetCurrentUserResponsePayload {
    username: String,
}

#[derive(ApiResponse)]
enum GetCurrentUserResponse {
    #[oai(status = 200)]
    Ok,
}

#[OpenApi(prefix_path = "/auth")]
impl AuthController {
    #[oai(path = "/login", method = "post")]
    async fn login(
        &self,
        data: Data<&Context>,
        credentials: Json<LoginRequest>,
    ) -> Result<LoginResponse, ResponseError> {
        AuthService::login(&data, &credentials.0.username, &credentials.0.password)
            .await
            .map(|token| {
                LoginResponse::Ok(Json(LoginResponsePayload {
                    token: token,
                    username: credentials.0.username,
                }))
            })
            .ok_or(ResponseError::Unauthorized)
    }

    #[oai(path = "/me", method = "get")]
    async fn me(&self, _data: JWTAuthorization) -> GetCurrentUserResponse {
        GetCurrentUserResponse::Ok
    }
}
