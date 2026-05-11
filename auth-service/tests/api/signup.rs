use auth_service::routes::SignupResponse;

use crate::helpers::{TestApp, get_random_email, get_random_password};

#[tokio::test]
async fn return_201_for_valid_input() {
    // boot the TestApp (server & client)
    let app = TestApp::new().await;

    // create valid test cases
    let test_cases = [
        serde_json::json!({
            "email": get_random_email(),
            "password": get_random_password(),
            "requires2FA": false
        }),
        serde_json::json!({
            "email": get_random_email(),
            "password": get_random_password(),
            "requires2FA": true
        })
    ];

    // POST-Request the '/signup' of the auth service with the created fields
    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status().as_u16(), 201, "Failed for input: {:?}", test_case);
        
        let expected_response = SignupResponse {
            message: "User created successfully".to_owned()
        };

        // asset that we get the correct Response body
        assert_eq!(
            response
                .json::<SignupResponse>()
                .await
                .expect("Could not deserialize response body to UserBody"),
            expected_response
        );
    }
}

/// Call the /signup route with various malformed inputs and assert that a 422 HTTP
/// Status Code is returned for each malformed input.
#[tokio::test]
async fn return_422_for_malformed_input() {
    let app = TestApp::new().await;

    // create invalid test cases
    let test_cases = [
        serde_json::json!({ // no email field
            "password": get_random_password(),
            "requires2FA": false
        }),
        serde_json::json!({ // no password field
            "email": get_random_email(),
            "requires2FA": false
        }),
        serde_json::json!({ // no requires2FA field
            "email": get_random_email(),
            "password": get_random_password(),
        }),
        serde_json::json!({ // only email field
            "email": get_random_email(),
        }),
        serde_json::json!({ // only password field
            "password": get_random_password(),
        }),
        serde_json::json!({ // only requires2FA field
            "requires2FA": false
        }),
    ];

    for test_case in test_cases.iter() {
        let response = app.post_signup(test_case).await;
        assert_eq!(response.status().as_u16(), 422, "Failed for input: {:?}", test_case);
    }
}