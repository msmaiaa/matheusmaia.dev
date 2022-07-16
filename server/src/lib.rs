use config::Context;
use controller::{AuthController, PostController, TagController};
//use controller::{AuthController, PostController, TagController};
use poem::{
    middleware::{AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;

mod common_types;
mod config;
mod controller;
mod database;
mod jwt;
mod repository;
mod service;

pub struct App {
    port: String,
    host: String,
}

pub type AppServer = AddDataEndpoint<CorsEndpoint<Route>, Context>;
impl App {
    pub fn new(host: String, port: String) -> App {
        App { host, port }
    }

    pub async fn make_server(self) -> AppServer {
        let db_pool = database::start_db(
            &100,
            &std::env::var("DATABASE_URL").expect("DATABASE_URL not set"),
        )
        .await;
        let ctx = config::Context::new(db_pool);
        let api_service = OpenApiService::new(
            (AuthController, TagController, PostController),
            "Api",
            "1.0",
        )
        .server(format!("http://{}:{}/api", self.host, self.port));
        let ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/api", api_service)
            .nest("/", ui)
            .with(Cors::new())
            .data(ctx);
        app
    }
}
