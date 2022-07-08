extern crate dotenv;
use std::sync::Arc;

use dotenv::dotenv;
use poem::{listener::TcpListener, middleware::Cors, middleware::Tracing, EndpointExt, Route};
use poem_openapi::OpenApiService;

mod common_types;
mod config;
mod controller;
mod db;
mod prisma;
mod repository;
mod service;

use controller::*;

struct Server {
    port: String,
    host: String,
}

impl Server {
    fn new() -> Server {
        let host = std::env::var("HOST")
            .unwrap_or("localhost".to_string())
            .to_string();
        let port = std::env::var("PORT")
            .unwrap_or("3000".to_string())
            .to_string();
        Server { port, host }
    }

    fn get_api_url(&self) -> String {
        format!("http://{}/api", self.get_server_url())
    }

    fn get_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let client = db::get_client().await.expect("Failed to get prisma client");
    let ctx = config::context::Context::new(Arc::new(client));
    let server = Server::new();
    let api_service =
        OpenApiService::new(AuthController, "Api", "1.0").server(server.get_api_url());
    let ui = api_service.swagger_ui();
    //tracing_subscriber::fmt::init();
    let app = Route::new()
        .nest("/api", api_service)
        .nest("/", ui)
        .with(Cors::new())
        //.with(Tracing)
        .data(ctx);

    poem::Server::new(TcpListener::bind(server.get_server_url()))
        .run(app)
        .await
}
