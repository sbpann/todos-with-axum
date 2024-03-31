use std::sync::Arc;

use crate::{modules::todos, ApplicationState};

use axum::{
    routing::{get, post, put},
    Router,
};

pub fn todos_router() -> Router<Arc<ApplicationState>> {
    let router = Router::new()
        .route("/:id", get(todos::controllers::get))
        .route("/", get(todos::controllers::list))
        .route("/", post(todos::controllers::post))
        .route("/:id", put(todos::controllers::put));

    router
}
