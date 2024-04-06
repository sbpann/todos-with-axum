mod configs;
mod constants;
mod modules;
mod router;
mod utils;
mod views;

use std::sync::Arc;

use axum::Router;
use configs::db::new_pg_pool;
use dotenvy::dotenv;
use sqlx::Postgres;

#[derive(Clone)]
pub struct ApplicationState {
    db_pool: sqlx::Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = Arc::new(ApplicationState {
        db_pool: new_pg_pool().await,
    });
    let app = Router::new()
        .nest("/todos", router::todos_router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
