use crate::helpers::TestApp;

/* Tokio's test macro is used to run the test in an async environment 
 * 
 * The tests are built around the following design pattern
 * - boot the TestApp (server & client) 
 * - create any fields needed for a given test / Request 
 * - send the Request
 * - obtain the Response to the Request
 * - evaluate the fields of the Request (assert_eq!() relevant fields)
 * --- did the auth service (server) Respond as we expect? */

#[tokio::test]
async fn root_returns_auth_ui() {
    // boot the TestApp (server & client)
    let app = TestApp::new().await;

    // GET-Request the '/' (root) of the auth service (server) and obtain the Response
    let response = app.get_root().await;

    // validate that the Response from the server matches our expectations
    assert_eq!(response.status().as_u16(), 200);
    assert_eq!(response.headers().get("content-type").unwrap(), "text/html");
}

#[tokio::test]
async fn signup_returns_success() {
    // boot the TestApp (server & client)
    let app = TestApp::new().await;

    // create fields for a successful signup
    let email = "some-email@gmail.com";
    let password = "some-password-string";
    let require2fa = "false";

    // POST-Request the '/signup' of the auth service with the created fields
    let response = app.post_signup(email, password, require2fa).await;

    assert_eq!(response.status().as_u16(), 201);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // body -- success message -- { "message": "User created successfully!"}
    assert_eq!(response.json::<String>().await.unwrap(), "User created successfully!")
}

#[tokio::test]
async fn signup_returns_invalid_email() {
    let app = TestApp::new().await;

    let email = "";
    let password = "some-password-string";
    let require2fa = "false";

    let response = app.post_signup(email, password, require2fa).await;

    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // body -- invalid message -- { "error": "error-message-string" }
    assert_eq!(response.json::<String>().await.unwrap(), "Invalid input. No email provided.")
}

#[tokio::test]
async fn signup_returns_invalid_password() {
    let app = TestApp::new().await;

    let email = "some-email@gmail.com";
    let password = "";
    let require2fa = "false";

    let response = app.post_signup(email, password, require2fa).await;

    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // body -- invalid message -- { "error": "error-message-string" }
    assert_eq!(response.json::<String>().await.unwrap(), "Invalid input. No password provided.")
}

#[tokio::test]
async fn signup_returns_invalid_2fa() {
    let app = TestApp::new().await;

    let email = "some-email@gmail.com";
    let password = "some-password-string";
    let require2fa = "";

    let response = app.post_signup(email, password, require2fa).await;

    assert_eq!(response.status().as_u16(), 400);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // body -- invalid message -- { "error": "error-message-string" }
    assert_eq!(response.json::<String>().await.unwrap(), "Invalid input. Define the 2FA requirement.")
}

#[tokio::test]
async fn signup_returns_duplicate_email() {
    let app = TestApp::new().await;

    let email = "some-email@gmail.com";
    let password = "some-password-string";
    let require2fa = "false";

    let _response = app.post_signup(email, password, require2fa).await;

    // if  response.status().as_u16() == 201 &&
    //     response.headers().get("content-type").unwrap() == "application/json" &&
    //     response.json::<String>().await.unwrap() == "User created successfully!" {
    //     // send request with duplicate email
    // }

    let email = "some-email@gmail.com";
    let password = "some-password-string";
    let require2fa = "false";

    let response = app.post_signup(email, password, require2fa).await;

    assert_eq!(response.status().as_u16(), 409);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // body -- invalid message -- { "error": "error-message-string" }
    assert_eq!(response.json::<String>().await.unwrap(), "An account with that email already exists.")
}

#[tokio::test]
async fn signup_returns_unprocessable() {
    let app = TestApp::new().await;

    let email = "some-email.com"; // invalid email format -- not able to be processed
    let password = "some-password-string";
    let require2fa = "false";

    let response = app.post_signup(email, password, require2fa).await;

    assert_eq!(response.status().as_u16(), 422);
    assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // body -- invalid message -- { "error": "error-message-string" }
    assert_eq!(response.json::<String>().await.unwrap(), "Content could not be processed. Field(s) entered incorrectly.")
}

// TODO: signup status code 500
// TODO: implement tests for all other routes (signup, login, logout, verify-2fa, and verify-token).
// -- for now, simply assert that each route returns a 200 HTTP status code.