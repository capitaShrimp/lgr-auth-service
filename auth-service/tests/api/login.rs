use crate::helpers::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn login_returns_success() {
    let app = TestApp::new().await;

    // let email = "some-email@gmail.com";
    // let password = "some-password-string";
    // let require2fa = "false";

    // let _signup_response = app.post_signup(email, password, require2fa).await;
    let login_response = app.post_login(&get_random_email(), &get_random_password()).await;
    assert_eq!(login_response.status().as_u16(), 200);
}