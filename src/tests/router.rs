#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::router::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use tower::ServiceExt;

    #[sqlx::test]
    async fn list(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"limit\":10,\"offset\":0,\"total\":0,\"items\":[]}");
    }
}
