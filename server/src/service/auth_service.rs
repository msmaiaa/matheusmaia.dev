use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

pub struct AuthService;

#[derive(Serialize, Deserialize)]
struct TokenData {
    username: String,
    iat: i64,
    exp: i64,
}

impl AuthService {
    pub fn create_access_token(username: String) -> String {
        let iat = Utc::now();
        let exp = iat + Duration::seconds(3600);
        let iat = iat.timestamp_millis();
        let exp = exp.timestamp_millis();

        let key =
            EncodingKey::from_secret(env::var("JWT_KEY").expect("JWT_KEY not set").as_bytes());
        let claims = TokenData {
            username: username.to_string(),
            iat,
            exp,
        };
        let header = Header::new(Algorithm::HS256);
        encode(&header, &claims, &key).expect("Failed to create access token")
    }
    pub fn encrypt(password: &str) -> String {
        //	the salt must have atleast 16 characters
        let salt = env::var("SALT").unwrap_or("123451234512345123451235".to_string());
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config)
            .expect("Failed to hash password")
    }
    pub fn compare_hash(password: &str, encrypted: &str) -> bool {
        let hashed_input = AuthService::encrypt(password);
        if hashed_input == encrypted.to_string() {
            return true;
        }
        false
    }
}
