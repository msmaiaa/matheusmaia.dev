use poem::Request;
use poem_openapi::{auth::Bearer, SecurityScheme};

use crate::service::AuthService;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TokenData {
    pub id: u32,
    pub iat: i64,
    pub exp: i64,
}

#[derive(SecurityScheme)]
#[oai(
    type = "bearer",
    key_name = "authorization",
    in = "header",
    checker = "check_jwt"
)]
pub struct JWTAuthorization(pub TokenData);

async fn check_jwt(_: &Request, bearer: Bearer) -> Option<TokenData> {
    AuthService::decode_token(&bearer.token)
}
