use std::env;

use axum::Router;

pub fn build_router<S>(modules: Vec<(&str, fn() -> Router<S>)>) -> Router<S>
where
    S: Clone + Send + Sync + 'static,
{
    let mut router = Router::new();
    for module in modules.iter() {
        router = router.nest(module.0, module.1())
    }
    router
}

pub fn build_listening_address() -> String {
    let port = env::var("PORT").unwrap_or("3000".to_string());
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_string());
    return format!("{}:{}", host, port);
}

pub async fn build_listener(address: String) -> tokio::net::TcpListener {
    tokio::net::TcpListener::bind(address).await.unwrap()
}
