mod constants;
mod modules;
mod router;
mod schema;
mod views;
mod configs;

use axum::Router;
use configs::db::PgDbPool;
use dotenvy::dotenv;

#[derive(Clone)]
pub struct ApplicationState {
    db_pool: PgDbPool
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let state = ApplicationState {db_pool: PgDbPool::new()};
    let app = Router::new()
    .nest("/todos", router::todos_router())
    .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
