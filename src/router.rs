use std::sync::Arc;

use crate::modules::todos;
use crate::configs::state;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn todos_router() -> Router<Arc<state::AppState>> {
    Router::new()
        .route("/:id", get(todos::controllers::get))
        .route("/", get(todos::controllers::list))
        .route("/", post(todos::controllers::post))
        .route("/:id", put(todos::controllers::put))
        .route("/:id", delete(todos::controllers::delete))
}
