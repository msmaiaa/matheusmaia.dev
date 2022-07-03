use crate::prisma::user;
use crate::{config::context::Context, service::AuthService};
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
    Ok(Json<String>),
    #[oai(status = 401)]
    Unauthorized,
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
            Some(token) => LoginResponse::Ok(Json(token)),
            None => LoginResponse::Unauthorized,
        }
    }
}
