use axum::{body::Body, extract::rejection::PathRejection, http::{Response, StatusCode}};

use crate::{constants::error_response::GENERIC_INTERNAL_SERVER_ERROR_RESPONSE, views::errors::from_error_kind};

pub fn build_response_from_path_rejection(path: &str, path_rejection_error: PathRejection) -> (StatusCode, Response<Body>) {
    match path_rejection_error {
            PathRejection::FailedToDeserializePathParams(error) => {
                from_error_kind(path.to_string(), error.kind())
            }
            PathRejection::MissingPathParams(error) => {
                println!("Error found {}", error);
                GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response()
            }
            _ => GENERIC_INTERNAL_SERVER_ERROR_RESPONSE.into_response(),
        }
}