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

pub fn build_paginated_query(
    query: &str,
    order_by: &Option<&str>,
    page: &Option<i64>,
    page_size: &Option<i64>,
) -> String {
    format!(
        "WITH Data_CTE
AS
(
{}
), 
Count_CTE 
AS 
(
	SELECT COUNT(*) AS TotalRows FROM Data_CTE
)
SELECT *
FROM Data_CTE
CROSS JOIN Count_CTE
ORDER BY {}
OFFSET ({} - 1) * {}
FETCH NEXT {} ROWS ONLY",
        query,
        order_by.unwrap_or("id"),
        page.unwrap_or(1),
        page_size.unwrap_or(10),
        page_size.unwrap_or(10)
    )
}
