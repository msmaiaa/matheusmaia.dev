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
pub struct JWTAuthorization(TokenData);

async fn check_jwt(req: &Request, _: Bearer) -> Option<TokenData> {
    let bearer = req.header("authorization").unwrap().replace("Bearer ", "");
    AuthService::decode_token(&bearer)
}
