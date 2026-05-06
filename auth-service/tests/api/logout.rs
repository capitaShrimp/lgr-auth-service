use crate::helpers::TestApp;

#[tokio::test]
async fn logout_returns_success() {
    let app = TestApp::new().await;

    let email = "some-email@gmail.com";
    let password = "some-password-string";
    let require2fa = "false";

    let _signup_response = app.post_signup(email, password, require2fa).await;
    let _login_response = app.post_login(email, password).await; // JWT cookie set

    // inspect the cookie here if desired
    for cookie in _login_response.cookies() {
        println!("Stored cookie → {}: {}", cookie.name(), cookie.value())
    }

    // make an authenicated request
    // the JWT cookie is sent automatically by the cookie store - no extra work needed
    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);
}