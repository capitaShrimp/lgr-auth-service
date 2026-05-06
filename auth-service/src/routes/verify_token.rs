use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify_token_handler() -> impl IntoResponse {
    (StatusCode::OK, "Token is valid").into_response()
}