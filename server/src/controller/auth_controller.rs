use crate::{common_types::TokenData, config::context::Context, service::AuthService};
use poem::{web::Data, Request};
use poem_openapi::{auth::Bearer, payload::Json, ApiResponse, Object, OpenApi, SecurityScheme};

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
    Ok(Json<GetCurrentUserResponsePayload>),
}

#[derive(SecurityScheme)]
#[oai(
    type = "bearer",
    key_name = "Authorization",
    in = "header",
    checker = "check_jwt"
)]
struct JWTAuthorization(TokenData);

async fn check_jwt(req: &Request, _: Bearer) -> Option<TokenData> {
    let bearer = req.header("Authorization").unwrap().replace("Bearer ", "");
    AuthService::decode_token(&bearer)
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
    async fn me(&self, data: Data<&Context>, auth: JWTAuthorization) -> GetCurrentUserResponse {
        GetCurrentUserResponse::Ok(Json(GetCurrentUserResponsePayload {
            username: auth.0.username,
        }))
    }
}
