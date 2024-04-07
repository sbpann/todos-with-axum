use crate::constants::error_response::GENERIC_INTERNAL_SERVER_ERROR_RESPONSE;
use axum::body::Body;
use axum::extract::path::ErrorKind;
use axum::http::Response;
use axum::response::IntoResponse;
use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct BadRequestErrorMessage {
    code: u16,
    message: String,
    path: String,
    comment: String,
}

impl BadRequestErrorMessage {
    fn invalid_type(path: String, expected_type: String) -> BadRequestErrorMessage {
        BadRequestErrorMessage {
            code: 400,
            message: "type of the following path is invalid".to_string(),
            path,
            comment: format!(
                "expected type: {}",
                BadRequestErrorMessage::get_type_description(expected_type)
            ),
        }
    }

    fn get_type_description(type_name: String) -> String {
        if type_name.contains('u') {
            return "unsigned interger".to_string();
        }
        if type_name.contains('i') {
            return "interger".to_string();
        }
        if type_name.contains('f') {
            return "number".to_string();
        }

        "unknown".to_string()
    }
}

pub fn from_error_kind(path: String, error_kind: &ErrorKind) -> (StatusCode, Response<Body>) {
    match error_kind {
        ErrorKind::ParseError {
            value: _,
            expected_type,
        } => (
            StatusCode::BAD_REQUEST,
            Json(BadRequestErrorMessage::invalid_type(
                path,
                (*expected_type).to_string(),
            ))
            .into_response(),
        ),
        &_ => GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
    }
}
