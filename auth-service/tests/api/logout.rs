use crate::helpers::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn logout_returns_success() {
    let app = TestApp::new().await;

    let _login_response = app.post_login(&get_random_email(), &get_random_password()).await; // JWT cookie set

    // inspect the cookie here if desired
    for cookie in _login_response.cookies() {
        println!("Stored cookie → {}: {}", cookie.name(), cookie.value())
    }

    // make an authenicated request
    // the JWT cookie is sent automatically by the cookie store - no extra work needed
    let logout_response = app.post_logout().await;
    assert_eq!(logout_response.status().as_u16(), 200);
}