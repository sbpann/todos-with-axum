#[cfg(test)]
mod tests {
    use rstest::*;
    use crate::constants::error_response::*;
    use axum::http::StatusCode;

    #[rstest]
    #[case(GENERIC_NOT_FOUND_ERROR_RESPONSE, StatusCode::NOT_FOUND)]
    #[case(GENERIC_INTERNAL_SERVER_ERROR_RESPONSE, StatusCode::INTERNAL_SERVER_ERROR)]
    fn generic_error_response_ok(#[case] error_message: JsonErrorMessage, #[case] status_code: StatusCode) {
        assert_eq!(error_message.into_response().0, status_code);
    }
}