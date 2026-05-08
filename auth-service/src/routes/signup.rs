use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use serde::Deserialize;

/// Handler for requests sent to the /signup route.
/// 
/// Uses Axum's Json extractor type which consumes the request body & deserializes it as JSON into
/// the target type -- the SignupRequest struct. By using the Json extractor, Axum automatically
/// asserts that the incoming request has a well-formatted JSON body. Otherwise, Axum rejects
/// the request with a 422 HTTP status code.
/// 
/// https://docs.rs/axum/latest/axum/extract/
pub async fn signup_handler(Json(_request): Json<SignupRequest>) -> impl IntoResponse {
    (StatusCode::OK, "User created successfully").into_response()
}

// automatically generate a Deserialize implementation for the SignupRequest struct
#[derive(Deserialize)] // https://docs.rs/serde/latest/serde/trait.Deserialize.html
pub struct SignupRequest {
    pub email: String,
    pub password: String,
    // used to serialize / deserialize a field with the given name instead of its Rust name
    #[serde(rename = "requires2FA")] // https://serde.rs/field-attrs.html
    pub requires_2fa: bool
}