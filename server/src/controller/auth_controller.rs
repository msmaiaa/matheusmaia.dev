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
        let found_user = data
            .prisma
            .user()
            .find_first(vec![user::username::equals(credentials.0.username.clone())])
            .exec()
            .await;
        match found_user {
            Ok(user) => match user {
                Some(user) => {
                    match AuthService::compare_hash(&credentials.0.password.clone(), &user.password)
                    {
                        true => LoginResponse::Ok(Json(AuthService::create_access_token(
                            credentials.0.username.clone(),
                        ))),
                        false => LoginResponse::Unauthorized,
                    }
                }
                None => {
                    let admin_username = std::env::var("ADMIN_USERNAME")
                        .unwrap_or("matheus".to_string())
                        .to_string();
                    if admin_username == credentials.0.username {
                        println!("teste");
                        let hashed_pass = AuthService::encrypt(&credentials.0.password);
                        let created = data
                            .prisma
                            .user()
                            .create(
                                user::username::set(credentials.0.username.clone()),
                                user::password::set(hashed_pass),
                                user::admin::set(true),
                                vec![],
                            )
                            .exec()
                            .await;
                        match created {
                            Ok(_) => {
                                return LoginResponse::Ok(Json(AuthService::create_access_token(
                                    credentials.0.username.clone(),
                                )));
                            }
                            Err(err) => {
                                println!("Error on user creation: {}", err);
                                return LoginResponse::Unauthorized;
                            }
                        }
                    }
                    return LoginResponse::Unauthorized;
                }
            },
            Err(_) => {
                return LoginResponse::Unauthorized;
            }
        }
    }
}
