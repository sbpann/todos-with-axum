#[cfg(test)]
mod tests {
    use std::{any::Any, sync::Arc};

    use crate::{configs::state::AppState, modules::todos::service::*};
    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test]
    async fn empty_list_ok(pool: PgPool) {
        let service = TodoService::new(Arc::new(AppState { db_pool: pool }));
        match service.list(10, 0).await {
            Err(error) => panic!("{}", error),
            Ok(todos) => {
                assert_eq!(todos.len(), 0);
            }
        }
    }

    #[sqlx::test]
    async fn create_ok(pool: PgPool) {
        let service = TodoService::new(Arc::new(AppState { db_pool: pool }));
        let random_title = Uuid::new_v4();
        let random_content = Uuid::new_v4();
        match service.create(format!("title-{}", random_title).as_str(), format!("content-{}", random_content).as_str()).await {
            
            Err(error) => panic!("{}", error),
            Ok(todo) => {
                assert_eq!(todo.id, 1);
                assert_eq!(todo.title, format!("title-{}", random_title));
                assert_eq!(todo.content, format!("content-{}", random_content));
            }
        }
    }

    #[sqlx::test(fixtures("mock_todo"))]
    async fn find_ok(pool: PgPool) {
        let service = TodoService::new(Arc::new(AppState { db_pool: pool }));
        match service.find(1).await {
            Err(error) => panic!("{}", error),
            Ok(todo) => {
                assert_eq!(todo.id, 1);
                assert_eq!(todo.title, "mock-title");
                assert_eq!(todo.content, "mock-content");
            }
        }
    }

    #[sqlx::test(fixtures("mock_todos"))]
    async fn list_ok(pool: PgPool) {
        let service = TodoService::new(Arc::new(AppState { db_pool: pool }));

        match service.list(10, 0).await {
            Err(error) => panic!("{}", error),
            Ok(todos) => {
                assert_eq!(todos.len(), 3);
                assert_eq!(todos[0].total, 3);

                assert_eq!(todos[0].id, 1);
                assert_eq!(todos[0].title, "mock-title-1");
                assert_eq!(todos[0].content, "mock-content-1");

                assert_eq!(todos[1].id, 2);
                assert_eq!(todos[1].title, "mock-title-2");
                assert_eq!(todos[1].content, "mock-content-2");

                assert_eq!(todos[2].id, 3);
                assert_eq!(todos[2].title, "mock-title-3");
                assert_eq!(todos[2].content, "mock-content-3");
            }
        }
    }

    #[sqlx::test(fixtures("mock_todos"))]
    async fn update_ok(pool: PgPool) {
        let service = TodoService::new(Arc::new(AppState { db_pool: pool }));

        match service.find(3).await {
            Err(error) => panic!("{}", error),
            Ok(todo) => {
                assert_eq!(todo.id, 3);
                assert_eq!(todo.title, "mock-title-3");
                assert_eq!(todo.content, "mock-content-3");
            }
        }

        match service.update(3, "foobar", "fozbaz").await {
            Err(error) => panic!("{}", error),
            Ok(todo) => {
                assert_eq!(todo.id, 3);
                assert_eq!(todo.title, "foobar");
                assert_eq!(todo.content, "fozbaz");
            }
        }

        match service.find(3).await {
            Err(error) => panic!("{}", error),
            Ok(todo) => {
                assert_eq!(todo.id, 3);
                assert_eq!(todo.title, "foobar");
                assert_eq!(todo.content, "fozbaz");
            }
        }
    }

    #[sqlx::test(fixtures("mock_todos"))]
    async fn delete_ok_find_err_not_found(pool: PgPool) {
        let service = TodoService::new(Arc::new(AppState { db_pool: pool }));

        match service.list(10, 0).await {
            Err(error) => panic!("{}", error),
            Ok(todos) => {
                assert_eq!(todos.len(), 3);
                assert_eq!(todos[0].total, 3);

                assert_eq!(todos[0].id, 1);
                assert_eq!(todos[0].title, "mock-title-1");
                assert_eq!(todos[0].content, "mock-content-1");

                assert_eq!(todos[1].id, 2);
                assert_eq!(todos[1].title, "mock-title-2");
                assert_eq!(todos[1].content, "mock-content-2");

                assert_eq!(todos[2].id, 3);
                assert_eq!(todos[2].title, "mock-title-3");
                assert_eq!(todos[2].content, "mock-content-3");
            }
        }

        match service.delete(1).await {
            Err(error) => panic!("{}", error),
            Ok(pq_query_result) => {
                assert_eq!(pq_query_result.rows_affected(), 1);
            }
        }

        match service.find(1).await {
            Err(error) => {
                assert_eq!(error.type_id(),
                sqlx::Error::RowNotFound.type_id());
            },
            Ok(todo) => {
                panic!("expected: not to exisit: {:#?}", todo);
            }
        }

        match service.list(10, 0).await {
            Err(error) => panic!("{}", error),
            Ok(todos) => {
                assert_eq!(todos.len(), 2);
                assert_eq!(todos[0].total, 2);

                assert_eq!(todos[0].id, 2);
                assert_eq!(todos[0].title, "mock-title-2");
                assert_eq!(todos[0].content, "mock-content-2");

                assert_eq!(todos[1].id, 3);
                assert_eq!(todos[1].title, "mock-title-3");
                assert_eq!(todos[1].content, "mock-content-3");
            }
        }
    }
}
