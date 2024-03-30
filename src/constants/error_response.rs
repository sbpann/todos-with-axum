use axum::{body::Body, http::{Response, StatusCode}, response::IntoResponse, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct DefaultErrorMessage {
    pub code: u16,
    pub message: &'static str,
}

pub struct JsonErrorMessage(StatusCode, Json<DefaultErrorMessage>);

impl JsonErrorMessage {
    pub fn into_response(self) ->  (StatusCode, Response<Body>) {
        (self.0, self.1.into_response())
    }   
}

const INTERNAL_ERROR_MESSAGE: &str = "internal server error";
const NOT_FOUND_ERROR_MESSAGE: &str = "not found";

const GENERIC_INTERNAL_ERROR: DefaultErrorMessage = DefaultErrorMessage {
    code: 500,
    message: INTERNAL_ERROR_MESSAGE,
};

const GENERIC_NOT_FOUND_ERROR: DefaultErrorMessage = DefaultErrorMessage {
    code: 404,
    message: NOT_FOUND_ERROR_MESSAGE,
};

pub const GENERIC_INTERNAL_SERVER_ERROR_RESPONSE: JsonErrorMessage = JsonErrorMessage(
    StatusCode::INTERNAL_SERVER_ERROR,
    Json(GENERIC_INTERNAL_ERROR),
);

pub const GENERIC_NOT_FOUND_ERROR_RESPONSE: JsonErrorMessage = JsonErrorMessage(
    StatusCode::NOT_FOUND,
    Json(GENERIC_NOT_FOUND_ERROR),
);
