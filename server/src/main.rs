extern crate dotenv;

use dotenv::dotenv;
use poem::listener::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    let host = std::env::var("HOST")
        .unwrap_or("localhost".to_string())
        .to_string();
    let port = std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .to_string();

    let app = app::App::new(host.clone(), port.clone());
    let server = app.make_server().await;
    let listener = TcpListener::bind(format!("{}:{}", host, port));
    poem::Server::new(listener).run(server).await;
    Ok(())
}
