use axum::{http::StatusCode, response::IntoResponse};

pub async fn signup_handler() -> impl IntoResponse {
    (StatusCode::CREATED, "User created successfully").into_response()
}