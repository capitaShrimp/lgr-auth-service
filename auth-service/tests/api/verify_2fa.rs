use crate::helpers::TestApp;

#[tokio::test]
async fn verify_2fa_returns_success() {
    let app = TestApp::new().await;

    let email = "some-email@gmail.com";
    let attempt_id = "some-login-attempt-id";
    let code_2fa = "some-2fa-code";

    let response = app.post_verify_2fa(email, attempt_id, code_2fa).await;
    assert_eq!(response.status().as_u16(), 200);
}