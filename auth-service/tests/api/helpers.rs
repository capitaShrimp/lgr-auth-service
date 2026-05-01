use std::{collections::HashMap, hash::Hash};
use auth_service::Application;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client
}

impl TestApp {
    /// Create an instance of the auth-service Application for integration testing.
    /// 
    /// This method configures and launches the auth-service and creates & returns an HTTP
    /// client for testing purposes.
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        // obtain a http routing string from the address that the application is bound / listening to
        let address = format!("http://{}", app.address.clone());

        // run the auth-service in a separate async task
        // -- avoids blocking the main test thread
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // create a reqwest http client instance
        let http_client = reqwest::Client::new();

        // create a new 'TestApp' instance and return it
        return TestApp {
            address,
            http_client
        }
    }

    /// GET-Request the root of the auth service and return the Response.
    pub async fn get_root(&self) -> reqwest::Response {
        // construct & send the GET Request, then return the Response
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute root request")
    }

    // TODO: implement helper functions for all other routes (signup, login, logout, verify-2fa, and verify-token)

    /// POST-Request the auth service to signup a new user and return the Response.
    /// 
    /// This route registers a new user.
    pub async fn post_signup(&self) -> reqwest::Response {
        // this will POST a body of `{"email":"user@example.com","password":"some-password","requires2FA":"false"}`
        let mut map = HashMap::new();
        map.insert("email", "user@example.com");
        map.insert("password", "some-password");
        map.insert("requires2FA", "false");

        // construct & send the POST Request, then return the Response
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute signup request")
    }

    /// POST-Request the auth service to login a user and return the Response.
    /// 
    /// This route authenticates the user and returns a JWT for the session.
    pub async fn post_login(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "user@example.com");
        map.insert("password", "some-password");

        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute login request")
    }
    
    /// POST-Request the logout page of the auth service and return the response.
    /// 
    /// This route ---
    pub async fn post_logout(&self) -> reqwest::Response {
        // TODO: finish this method with parameters / body
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute logout request")
    }

    /// POST-Request the auth service to verify a 2FA token and return the Response.
    /// 
    /// This route verifies a 2FA token.
    pub async fn post_verify_2fa(&self) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", "user@example.com");
        map.insert("loginAttemptId", "id-string");
        map.insert("2FACode", "2FA-string");

        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute verify-2fa request")
    }
    /// POST-Request the verify-token page of the auth service and return the response.
    pub async fn post_verify_token(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .send()
            .await
            .expect("Failed to execute verify-token request")
    }
}