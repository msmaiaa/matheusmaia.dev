use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub type DbPool = Pool<Postgres>;
pub async fn start_db(max_connections: &u32, connection_uri: &str) -> Pool<Postgres> {
    println!("{}", connection_uri.clone());
    PgPoolOptions::new()
        .max_connections(*max_connections)
        .connect(connection_uri)
        .await
        .unwrap_or_else(|err| {
            panic!(
                "Couldn't make the initial connection with the database {}",
                err.to_string()
            )
        })
}
