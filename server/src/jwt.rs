use poem::Request;
use poem_openapi::{auth::Bearer, SecurityScheme};

use crate::{common_types::TokenData, service::AuthService};

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
