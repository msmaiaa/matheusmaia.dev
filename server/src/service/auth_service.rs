use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header};
use std::env;

use crate::jwt::TokenData;

pub struct AuthService;

impl AuthService {
    pub async fn login(
        ctx: &crate::config::Context,
        username: &str,
        password: &str,
    ) -> Option<String> {
        // TODO: automatically create the admin user on container startup and remove this logic
        let user_repo =
            crate::repository::user_repository::UserRepository::new(ctx.db_pool.clone());
        let found_user = user_repo.find_by_username(&username).await;
        match found_user {
            Some(user) => match AuthService::compare_hash(&password, &user.password) {
                true => return Some(AuthService::create_access_token(user.id)),
                false => return None,
            },
            None => {
                if !AuthService::is_admin_username(&username) {
                    return None;
                }
                let hashed_pass = AuthService::encrypt(&password);
                user_repo
                    .create(&username, &hashed_pass, true)
                    .await
                    .map(|created_id| Some(AuthService::create_access_token(created_id)))
                    .unwrap_or(None)
            }
        }
    }

    fn is_admin_username(username: &str) -> bool {
        let admin_username = std::env::var("ADMIN_USERNAME")
            .unwrap_or("matheus".to_string())
            .to_string();
        username == admin_username
    }

    pub fn create_access_token(id: i32) -> String {
        let iat = Utc::now();
        let exp = iat + Duration::seconds(3600);
        let iat = iat.timestamp_millis();
        let exp = exp.timestamp_millis();

        let key =
            EncodingKey::from_secret(env::var("JWT_KEY").expect("JWT_KEY not set").as_bytes());
        let claims = TokenData { id, iat, exp };
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &key).expect("Failed to create access token")
    }

    pub fn decode_token(token: &str) -> Option<TokenData> {
        let key =
            DecodingKey::from_secret(env::var("JWT_KEY").expect("JWT_KEY not set").as_bytes());
        let res = decode::<TokenData>(&token, &key, &jsonwebtoken::Validation::default());
        match res {
            Ok(data) => Some(data.claims),
            Err(_) => None,
        }
    }

    pub fn encrypt(password: &str) -> String {
        //	the salt must have atleast 16 characters
        let salt = env::var("SALT").unwrap_or("123451234512345123451235".to_string());
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
            .expect("Failed to hash password")
    }

    pub fn compare_hash(password: &str, encrypted: &str) -> bool {
        AuthService::encrypt(&password) == encrypted.to_string()
    }
}
