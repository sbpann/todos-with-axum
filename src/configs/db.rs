use sqlx::{postgres::PgPoolOptions, Postgres};

pub async fn new_pg_pool(database_url: &str) -> sqlx::Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(50)
        .connect(database_url)
        .await
        .unwrap_or_else(|error| panic!("Could not build connection pool: {}", error))
}
