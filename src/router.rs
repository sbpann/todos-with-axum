use crate::{modules::todos, ApplicationState};

use axum::{
    routing::{get, post},
    Router,
};

pub fn todos_router() -> Router<ApplicationState> {
    let router = Router::new()
        .route("/:id", get(todos::controllers::get))
        .route("/", get(todos::controllers::list))
        .route("/", post(todos::controllers::post));

    router
}
