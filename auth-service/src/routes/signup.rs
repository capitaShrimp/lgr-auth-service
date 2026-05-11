use axum::{extract::{Json, State}, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use crate::{app_state::AppState, domain::User};

/// Handler for requests sent to the /signup route.
/// 
/// Uses Axum's Json extractor type which consumes the request body & deserializes it as JSON into
/// the target type -- the SignupRequest struct. By using the Json extractor, Axum automatically
/// asserts that the incoming request has a well-formatted JSON body. Otherwise, Axum rejects
/// the request with a 422 HTTP status code.
/// 
/// Uses Axum's State extractor type to pass in AppState. This is how we can access our UserStore.
/// 
/// https://docs.rs/axum/latest/axum/extract/
/// 
/// https://docs.rs/axum/latest/axum/extract/struct.State.html
pub async fn signup_handler(State(state): State<AppState>, // order matters!
                            Json(request): Json<SignupRequest>) -> impl IntoResponse {

    // create a new 'User' instance using the data in the Request
    let user = User::new(request.email, request.password, request.requires_2fa);

    // lock the user_store for writing
    let mut user_store = state.user_store.write().await;

    // add the new 'User' to the UserStore & simply .unwrap() the 'Result' for now
    user_store.add_user(user).unwrap();

    // craft the Response
    let signup_response = Json(SignupResponse {
        message: "User created successfully".to_string()
    });

    // return the Response
    (StatusCode::CREATED, signup_response)
}

/// Response body for successful signup.
/// Derive Serialize for the signup_handler.
/// Derive Deserialize, PartialEq, and Debug for the test api.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct SignupResponse {
    pub message: String
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