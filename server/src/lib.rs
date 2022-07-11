use config::context::Context;
use controller::{AuthController, TagController};
use poem::{
    middleware::{AddDataEndpoint, Cors, CorsEndpoint},
    EndpointExt, Route,
};
use poem_openapi::OpenApiService;
use std::sync::Arc;

mod common_types;
mod config;
mod controller;
mod db;
mod jwt;
mod prisma;
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
        let prisma_client = db::get_client().await.expect("Failed to get prisma client");
        let ctx = config::context::Context::new(Arc::new(prisma_client));
        let api_service = OpenApiService::new((AuthController, TagController), "Api", "1.0")
            .server(format!("http://{}:{}/api", self.host, self.port));
        let ui = api_service.swagger_ui();
        let app = Route::new()
            .nest("/api", api_service)
            .nest("/", ui)
            .with(Cors::new())
            //.with(Tracing)
            .data(ctx);
        app
    }
}
