use crate::{config::context::Context, jwt::JWTAuthorization, service::AuthService};
use poem::web::Data;
use poem_openapi::{payload::Json, ApiResponse, Object, OpenApi};

pub struct AuthController;

#[derive(Object)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(ApiResponse)]
enum LoginResponse {
    #[oai(status = 200)]
    Ok(Json<LoginResponsePayload>),
    #[oai(status = 401)]
    Unauthorized,
}

#[derive(Object)]
struct LoginResponsePayload {
    token: String,
    username: String,
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
    async fn login(&self, data: Data<&Context>, credentials: Json<LoginRequest>) -> LoginResponse {
        let token = AuthService::login(
            data.prisma.to_owned(),
            &credentials.0.username,
            &credentials.0.password,
        )
        .await;
        match token {
            Some(token) => LoginResponse::Ok(Json(LoginResponsePayload {
                token,
                username: credentials.0.username,
            })),
            None => LoginResponse::Unauthorized,
        }
    }

    #[oai(path = "/me", method = "get")]
    async fn me(&self, _data: JWTAuthorization) -> GetCurrentUserResponse {
        GetCurrentUserResponse::Ok
    }
}
