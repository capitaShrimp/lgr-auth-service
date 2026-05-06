use axum::{http::StatusCode, response::IntoResponse};

pub async fn login_handler() -> impl IntoResponse {
    (StatusCode::OK, "Login successful").into_response()
}