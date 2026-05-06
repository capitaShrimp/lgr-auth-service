use axum::{http::StatusCode, response::IntoResponse};

pub async fn logout_handler() -> impl IntoResponse {
    (StatusCode::OK, "Logout successful").into_response()
}