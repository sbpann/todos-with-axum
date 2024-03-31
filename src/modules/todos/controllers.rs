use std::sync::Arc;

use super::{service::TodoService, views};
use crate::utils::error::build_response_from_path_rejection;
use crate::{
    constants::error_response::{
        GENERIC_INTERNAL_SERVER_ERROR_RESPONSE, GENERIC_NOT_FOUND_ERROR_RESPONSE,
    },
    ApplicationState,
};
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

pub async fn get(
    State(state): State<Arc<ApplicationState>>,
    id: Result<Path<i32>, PathRejection>,
) -> (StatusCode, impl IntoResponse) {
    let id = match id {
        Err(path_rejection_error) => {
            return build_response_from_path_rejection("id", path_rejection_error)
        }
        Ok(value) => value.0,
    };

    let todo_service = TodoService::new(state);

    match todo_service.find(id) {
        Err(_) => return GENERIC_NOT_FOUND_ERROR_RESPONSE.into_response(),
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

pub async fn list(State(state): State<Arc<ApplicationState>>) -> (StatusCode, impl IntoResponse) {
    let todo_service = TodoService::new(state);
    let mut todos: Vec<views::Todo> = vec![];

    match todo_service.list() {
        Err(_) => return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(list) => {
            for todo in list.iter() {
                let todo_clone = todo.clone();
                todos.push(views::Todo {
                    id: todo.id,
                    title: todo_clone.title,
                    content: todo_clone.content,
                });
            }
        }
    }

    (StatusCode::OK, Json(views::Todos { todos }).into_response())
}

pub async fn post(
    State(state): State<Arc<ApplicationState>>,
    Json(request): Json<TodoRequest>,
) -> (StatusCode, impl IntoResponse) {
    let todo_service = TodoService::new(state);

    match todo_service.create(&request.title, &request.content) {
        Err(_) => return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        Ok(todo) => {
            let view = views::Todo {
                id: todo.id,
                title: todo.title,
                content: todo.content,
            };
            return (StatusCode::CREATED, Json(view).into_response());
        }
    };
}

pub async fn put(
    State(state): State<Arc<ApplicationState>>,
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
    let result = todo_service.find(id);

    match result {
        Err(_) => GENERIC_NOT_FOUND_ERROR_RESPONSE.into_response(),
        Ok(todo) => {
            let updated_todo = match todo_service.update(todo.id, &request.title, &request.content)
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
