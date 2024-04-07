use std::cmp;
use std::sync::Arc;

use super::{service::TodoService, views};
use crate::configs::state::AppState;
use crate::utils::error::build_response_from_path_rejection;
use crate::constants::error_response::{
        GENERIC_INTERNAL_SERVER_ERROR_RESPONSE, GENERIC_NOT_FOUND_ERROR_RESPONSE,
    };
use axum::body::Body;
use axum::extract::Query;
use axum::{
    extract::{rejection::PathRejection, Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoRequest {
    title: String,
    content: String,
}

#[derive(Deserialize)]
pub struct Pagination {
    limit: Option<i64>,
    offset: Option<i64>,
}

pub async fn get(
    State(state): State<Arc<AppState>>,
    id: Result<Path<i32>, PathRejection>,
) -> (StatusCode, impl IntoResponse) {
    let id = match id {
        Err(path_rejection_error) => {
            return build_response_from_path_rejection("id", path_rejection_error)
        }
        Ok(value) => value.0,
    };

    let todo_service = TodoService::new(state);

    match todo_service.find(id).await {
        Err(_) => GENERIC_NOT_FOUND_ERROR_RESPONSE.into_response(),
        Ok(todo) => {
            let view = views::Todo {
                id: todo.id,
                title: todo.title,
                content: todo.content,
            };
            (StatusCode::OK, Json(view).into_response())
        }
    }
}

pub async fn list(
    State(state): State<Arc<AppState>>,
    Query(pagination): Query<Pagination>,
) -> (StatusCode, impl IntoResponse) {
    let limit = cmp::min(pagination.limit.unwrap_or(10), 10);
    let offset = pagination.offset.unwrap_or(0);

    let todo_service = TodoService::new(state);
    let mut todos: Vec<views::Todo> = vec![];
    let total: i64;

    match todo_service.list(limit, offset).await {
        Err(_) => return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(list) => {
            total = match list.is_empty() {
                true => 0,
                false => list[0].total
            };
            for todo in list.iter() {
                todos.push(views::Todo {
                    id: todo.id,
                    title: todo.title.to_string(),
                    content: todo.content.to_string(),
                });
            }
        }
    }

    (
        StatusCode::OK,
        Json(crate::views::pagination::Pagination {
            limit,
            offset,
            total,
            items: todos,
        })
        .into_response(),
    )
}

pub async fn post(
    State(state): State<Arc<AppState>>,
    Json(request): Json<TodoRequest>,
) -> (StatusCode, impl IntoResponse) {
    let todo_service = TodoService::new(state);

    match todo_service.create(&request.title, &request.content).await {
        Err(_) => GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(todo) => {
            let view = views::Todo {
                id: todo.id,
                title: todo.title,
                content: todo.content,
            };
            (StatusCode::CREATED, Json(view).into_response())
        }
    }
}

pub async fn put(
    State(state): State<Arc<AppState>>,
    id: Result<Path<i32>, PathRejection>,
    Json(request): Json<TodoRequest>,
) -> (StatusCode, impl IntoResponse) {
    let id = match id {
        Err(path_rejection_error) => {
            return build_response_from_path_rejection("id", path_rejection_error)
        }
        Ok(value) => value.0,
    };

    let todo_service = TodoService::new(state);
    let result = todo_service.find(id).await;

    match result {
        Err(sqlx::Error::RowNotFound) => GENERIC_NOT_FOUND_ERROR_RESPONSE.into_response(),
        Err(_) => GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(todo) => {
            let updated_todo = match todo_service
                .update(todo.id, &request.title, &request.content)
                .await
            {
                Err(_) => return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
                Ok(updated_todo) => updated_todo,
            };

            let view = views::Todo {
                id: updated_todo.id,
                title: updated_todo.title,
                content: updated_todo.content,
            };
            (StatusCode::OK, Json(view).into_response())
        }
    }
}
pub async fn delete(
    State(state): State<Arc<AppState>>,
    id: Result<Path<i32>, PathRejection>,
) -> (StatusCode, impl IntoResponse) {
    let id = match id {
        Err(path_rejection_error) => {
            return build_response_from_path_rejection("id", path_rejection_error)
        }
        Ok(value) => value.0,
    };

    let todo_service = TodoService::new(state);
    let find_result = match todo_service.find(id).await {
        Err(sqlx::Error::RowNotFound) => return GENERIC_NOT_FOUND_ERROR_RESPONSE.into_response(),
        Err(_) => return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(result) => result,
    };

    match todo_service.delete(find_result.id).await {
        Err(_) => GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(pg_query_result) => {
            if pg_query_result.rows_affected() != 1_u64 {
                return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response();
            }
            (StatusCode::NO_CONTENT, Body::empty().into_response())
        }
    }
}
