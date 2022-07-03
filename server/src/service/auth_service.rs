use argon2::{self, Config};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

use crate::prisma::{self};

pub struct AuthService;

#[derive(Serialize, Deserialize)]
struct TokenData {
    username: String,
    iat: i64,
    exp: i64,
}

impl AuthService {
    pub async fn login(
        prisma: std::sync::Arc<prisma::PrismaClient>,
        username: &str,
        password: &str,
    ) -> Option<String> {
        let user_repo = crate::repository::user_repository::UserRepository::new(prisma.clone());
        let found_user = user_repo.find_by_username(username).await;
        match found_user {
            Ok(user) => match user {
                Some(user) => match AuthService::compare_hash(&password.clone(), &user.password) {
                    true => return Some(AuthService::create_access_token(username.to_string())),
                    false => return None,
                },
                None => {
                    let admin_username = std::env::var("ADMIN_USERNAME")
                        .unwrap_or("matheus".to_string())
                        .to_string();
                    if admin_username == username {
                        println!("teste");
                        let hashed_pass = AuthService::encrypt(&password);
                        let created = user_repo.create(username, &hashed_pass, true).await;
                        match created {
                            Ok(_) => {
                                return Some(AuthService::create_access_token(
                                    username.to_string(),
                                ));
                            }
                            Err(err) => {
                                println!("Error on user creation: {}", err);
                                return None;
                            }
                        }
                    }
                    return None;
                }
            },
            Err(_) => {
                return None;
            }
        };
    }
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
