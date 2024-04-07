#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{extract::State, routing::get, Router};

    use crate::utils::app::*;

    #[derive(Clone)]
    struct TestState {}

    #[test]
    fn build_listening_address_default_ok() {
        assert_eq!(build_listening_address(), "0.0.0.0:3000");
    }

    #[test]
    fn build_listening_address_with_env_ok() {
        temp_env::with_vars(
            [("PORT", Some("8080")), ("HOST", Some("localhost"))],
            || {
                assert_eq!(build_listening_address(), "localhost:8080");
            },
        );
    }

    #[test]
    fn build_router_empty_ok() {
        let modules: Vec<(&str, fn() -> Router<Arc<TestState>>)> = vec![];
        let _ = build_router(modules);
    }

    #[test]
    fn build_router_ok() {
        let mut modules: Vec<(&str, fn() -> Router<Arc<TestState>>)> = vec![];

        async fn test_handler(State(_): State<Arc<TestState>>) -> &'static str {
            "test"
        }
        fn test_router() -> Router<Arc<TestState>> {
            Router::new().route("/", get(test_handler))
        }

        modules.push(("/tests", test_router));
        let _ = build_router(modules);
    }

    #[test]
    #[should_panic]
    fn build_router_err_duplicated_path() {
        let mut modules: Vec<(&str, fn() -> Router<Arc<TestState>>)> = vec![];

        async fn test_handler(State(_): State<Arc<TestState>>) -> &'static str {
            "test"
        }
        fn test_router() -> Router<Arc<TestState>> {
            Router::new().route("/", get(test_handler))
        }

        modules.push(("/tests", test_router));
        modules.push(("/tests", test_router));
        let _ = build_router(modules);
    }

    #[tokio::test]
    async fn build_listener_ok() {
        let listener = build_listener("localhost:9999".to_string()).await;
        let local_addr_result = listener.local_addr();
        assert!(!local_addr_result.is_err());
        let socket_address = local_addr_result.unwrap();
        assert_eq!(socket_address.port(), 9999);
    }
}
