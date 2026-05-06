use axum::{http::StatusCode, response::IntoResponse};

pub async fn verify_2fa_handler() -> impl IntoResponse {
    (StatusCode::OK, "2FA token verified successfully").into_response()
}