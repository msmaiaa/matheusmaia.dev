use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub async fn start_db(max_connections: &u32, connection_uri: &str) -> Pool<MySql> {
    println!("{}", connection_uri.clone());
    MySqlPoolOptions::new()
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
