use std::{env, sync::Arc};

use super::db::new_pg_pool;
use sqlx::Postgres;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: sqlx::Pool<Postgres>,
}

pub async fn build_state() -> Arc<AppState> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|error| panic!("Found error on initializing database: {}", error));
    Arc::new(AppState {
        db_pool: new_pg_pool(&database_url).await,
    })
}
