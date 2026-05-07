use crate::helpers::{get_random_email, get_random_password, TestApp};

#[tokio::test]
async fn verify_token_returns_success() {
    let app = TestApp::new().await;

    // let email = "some-email@gmail.com";
    // let password = "some-password-string";
    // let require2fa = "false";

    // let _signup_response = app.post_signup(email, password, require2fa).await;
    let _login_response = app.post_login(&get_random_email(), &get_random_password()).await; // JWT cookie set

    // inspect the cookie here if desired
    for cookie in _login_response.cookies() {
        println!("Stored cookie → {}: {}", cookie.name(), cookie.value());
        
        // identified the 'jwt' cookie from the Response -- post for verification
        if cookie.name() == "jwt" {
            let response = app.post_verify_token(cookie.value()).await;
            assert_eq!(response.status().as_u16(), 200);
            return
        }
    }

    println!("Error: failed to identify JWT cookie");
}