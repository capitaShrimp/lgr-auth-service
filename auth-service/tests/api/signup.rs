use crate::helpers::TestApp;

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
    // assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    // // body -- success message -- { "message": "User created successfully!"}
    // assert_eq!(response.json::<String>().await.unwrap(), "User created successfully!")
}