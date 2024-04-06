use std::env;

use sqlx::{postgres::PgPoolOptions, Postgres};

fn build_database_url() -> String {
    let error_message = "Cannot inititalize database config";
    let user = env::var("DATABASE_USER").unwrap_or_else(|err| panic!("{}: {}", error_message, err));
    let password =
        env::var("DATABASE_PASSWORD").unwrap_or_else(|err| panic!("{}: {}", error_message, err));
    let name = env::var("DATABASE_NAME").unwrap_or_else(|err| panic!("{}: {}", error_message, err));
    let host = env::var("DATABASE_HOST").unwrap_or_else(|err| panic!("{}: {}", error_message, err));
    let port = env::var("DATABASE_PORT").unwrap_or_else(|err| panic!("{}: {}", error_message, err));

    let database_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user, password, host, port, name
    );

    database_url
}

pub async fn new_pg_pool() -> sqlx::Pool<Postgres>{
    let db = PgPoolOptions::new()
    .max_connections(50)
    .connect(build_database_url().as_str())
    .await
    .unwrap_or_else(|error| panic!("Could not build connection pool: {}", error));

    db
}