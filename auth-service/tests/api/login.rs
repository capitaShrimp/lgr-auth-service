use crate::helpers::TestApp;

#[tokio::test]
async fn login_returns_success() {
    let app = TestApp::new().await;

    let email = "some-email@gmail.com";
    let password = "some-password-string";
    let require2fa = "false";

    let _signup_response = app.post_signup(email, password, require2fa).await;
    let login_response = app.post_login(email, password).await;

    assert_eq!(login_response.status().as_u16(), 200);
    // assert_eq!(login_response.headers().get("content-type").unwrap(), "application/json");
    // // body -- success message -- { "message": "User created successfully!"}
    // assert_eq!(login_response.json::<String>().await.unwrap(), "User created successfully!")
}