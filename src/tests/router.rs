#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use crate::router::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use sqlx::PgPool;
    use tower::ServiceExt;

    #[sqlx::test(fixtures("drop_todos_table"))]
    async fn db_error(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let mut response = todo_router.clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        let mut body = response.into_body().collect().await.unwrap().to_bytes();
        let mut string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"code\":500,\"message\":\"internal server error\"}"
        );

        response = todo_router.clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"does-not-matter\",\"content\":\"dose-not-matter\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"code\":500,\"message\":\"internal server error\"}"
        );

        response = todo_router.clone()
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method("PUT")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"updated-test-title\",\"content\":\"updated-test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"code\":500,\"message\":\"internal server error\"}"
        );

        response = todo_router.clone()
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"code\":500,\"message\":\"internal server error\"}"
        );
    }

    #[sqlx::test]
    async fn list_ok(pg_pool: PgPool) {
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
        assert_eq!(
            string_body,
            "{\"limit\":10,\"offset\":0,\"total\":0,\"items\":[]}"
        );
    }

    #[sqlx::test]
    async fn list_with_limit_offset_ok(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/?limit=5&&offset=20")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"limit\":5,\"offset\":20,\"total\":0,\"items\":[]}"
        );
    }

    #[sqlx::test]
    async fn list_with_over_limit_should_reduce_to_10(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/?limit=9999&&offset=0")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"limit\":10,\"offset\":0,\"total\":0,\"items\":[]}"
        );
    }

    #[sqlx::test]
    async fn create_ok(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"test-title\",\"content\":\"test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}"
        );
    }

    #[sqlx::test]
    async fn find_not_found(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/9999")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":404,\"message\":\"not found\"}");
    }

    #[sqlx::test]
    async fn find_err_non_numeric_id(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/foobar")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":400,\"message\":\"type of the following path is invalid\",\"path\":\"id\",\"comment\":\"expected type: interger\"}");
    }

    #[sqlx::test]
    async fn create_and_find_ok(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let mut response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"test-title\",\"content\":\"test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let mut body = response.into_body().collect().await.unwrap().to_bytes();
        let mut string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}"
        );

        response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method("GET")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"test-title\",\"content\":\"test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}"
        );
    }

    #[sqlx::test]
    async fn create_and_list_ok(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let mut response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"test-title\",\"content\":\"test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let mut body = response.into_body().collect().await.unwrap().to_bytes();
        let mut string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}"
        );

        response = todo_router
            .clone()
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
        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"limit\":10,\"offset\":0,\"total\":1,\"items\":[{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}]}");
    }

    #[sqlx::test]
    async fn create_and_update_ok(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let mut response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"test-title\",\"content\":\"test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let mut body = response.into_body().collect().await.unwrap().to_bytes();
        let mut string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}"
        );

        response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method("PUT")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"updated-test-title\",\"content\":\"updated-test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"updated-test-title\",\"content\":\"updated-test-content\"}"
        );
    }

    #[sqlx::test]
    async fn update_err_non_numeric_id(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/foobar")
                    .method("PUT")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"updated-test-title\",\"content\":\"updated-test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":400,\"message\":\"type of the following path is invalid\",\"path\":\"id\",\"comment\":\"expected type: interger\"}");
    }

    #[sqlx::test]
    async fn update_err_not_found(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/9999")
                    .method("PUT")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"does-not-matter\",\"content\":\"does-not-matter\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":404,\"message\":\"not found\"}");
    }

    #[sqlx::test]
    async fn create_and_delete_find_not_found(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let mut response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .method("POST")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"test-title\",\"content\":\"test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let mut body = response.into_body().collect().await.unwrap().to_bytes();
        let mut string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(
            string_body,
            "{\"id\":1,\"title\":\"test-title\",\"content\":\"test-content\"}"
        );

        response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NO_CONTENT);
        body = response.into_body().collect().await.unwrap().to_bytes();
        string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "");

        response = todo_router
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/1")
                    .method("GET")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":404,\"message\":\"not found\"}");
    }

    #[sqlx::test]
    async fn delete_err_non_numeric_id(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/foobar")
                    .method("DELETE")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        "{\"title\":\"updated-test-title\",\"content\":\"updated-test-content\"}",
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":400,\"message\":\"type of the following path is invalid\",\"path\":\"id\",\"comment\":\"expected type: interger\"}");
    }

    #[sqlx::test]
    async fn delete_err_not_found(pg_pool: PgPool) {
        let test_app_state = Arc::new(crate::configs::state::AppState { db_pool: pg_pool });
        let todo_router = todos_router().with_state(test_app_state);

        let response = todo_router
            .oneshot(
                Request::builder()
                    .uri("/9999")
                    .method("DELETE")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let string_body = std::str::from_utf8(&body).unwrap();
        assert_eq!(string_body, "{\"code\":404,\"message\":\"not found\"}");
    }
}
