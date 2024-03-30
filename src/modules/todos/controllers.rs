use super::{service::TodoService, views};
use crate::views::errors::from_error_kind;
use crate::{
    constants::
        error_response::{
            GENERIC_INTERNAL_SERVER_ERROR_RESPONSE, GENERIC_NOT_FOUND_ERROR_RESPONSE,
        }
    ,
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
    State(state): State<ApplicationState>,
    id: Result<Path<i32>, PathRejection>,
) -> (StatusCode, impl IntoResponse) {
    match id {
        Ok(value) => {
            let todo_service = TodoService::new(state);

            match todo_service.find(value.0) {
                Ok(todo) => {
                    let view = views::Todo {
                        id: todo.id,
                        title: todo.title,
                        content: todo.content,
                    };
                    (StatusCode::OK, Json(view).into_response())
                }
                Err(_) => GENERIC_NOT_FOUND_ERROR_RESPONSE.into_response(),
            }
        }
        Err(path_rejection_error) => match path_rejection_error {
            PathRejection::FailedToDeserializePathParams(error) => {
                from_error_kind("id".to_string(), error.kind())
            }
            PathRejection::MissingPathParams(error) => {
                println!("Error found {}", error);
                GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response()
            }
            _ => GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        },
    }
}

pub async fn list(State(state): State<ApplicationState>) -> (StatusCode, impl IntoResponse) {
    let todo_service = TodoService::new(state);
    let mut todos: Vec<views::Todo> = vec![];

    match todo_service.list() {
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
        Err(_) => {}
    }

    (StatusCode::OK, Json(views::Todos { todos }).into_response())
}

pub async fn post(
    State(state): State<ApplicationState>,
    Json(request): Json<TodoRequest>,
) -> (StatusCode, impl IntoResponse) {
    let todo_service = TodoService::new(state);

    match todo_service.create(&request.title, &request.content) {
        Ok(todo) => {
            let view = views::Todo {
                id: todo.id,
                title: todo.title,
                content: todo.content,
            };
            return (StatusCode::CREATED, Json(view).into_response());
        }
        Err(_) => {
            return GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response();
        }
    };
}
