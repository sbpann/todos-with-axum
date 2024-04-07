mod configs;
mod constants;
mod modules;
mod router;
mod utils;
mod views;
mod tests;

use std::sync::Arc;

use axum::Router;
use configs::state;
use dotenvy::dotenv;
use utils::app;

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let mut module_list: Vec<(&str, fn() -> Router<Arc<state::AppState>>)>  = vec![];
    module_list.push(("/todos", router::todos_router));

    let router = app::build_router(module_list);

    let listener = app::build_listener(app::build_listening_address()).await;
    axum::serve(listener, router.with_state(state::build_state().await)).await.unwrap();
}

