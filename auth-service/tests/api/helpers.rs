use std::{collections::HashMap};
use auth_service::Application;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client
}

impl TestApp {
    /// Create an instance of the auth-service Application for integration testing.
    /// 
    /// This method configures and launches the auth-service (server) and
    /// creates & returns an HTTP client for testing purposes.
    pub async fn new() -> Self {
        let app = Application::build("127.0.0.1:0")
            .await
            .expect("Failed to build app");

        // obtain an http routing string from the address that the application is bound / listening to
        let address = format!("http://{}", app.address.clone());

        // run the auth-service in a separate async task
        // -- avoids blocking the main test thread
        #[allow(clippy::let_underscore_future)]
        let _ = tokio::spawn(app.run());

        // create a reqwest http client with an automatic, persistent cookie store
        let http_client = reqwest::Client::builder()
            .cookie_store(true)
            .build()
            .unwrap(); // TODO: implement error handling

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

    /// POST-Request the auth service to signup a new user and return the Response.
    /// 
    /// This route registers a new user.
    pub async fn post_signup<Body>(&self, body: &Body) -> reqwest::Response 
        // body is generic over any type that implements Serde's Serialize trait
        where Body: serde::Serialize
    {
        // construct & send the POST Request, then return the Response
        self.http_client
            .post(&format!("{}/signup", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute signup request")
    }

    /// POST-Request the auth service to login a user and return the Response.
    /// 
    /// This route authenticates the user and returns a JWT for the session.
    pub async fn post_login(&self, email: &str, password: &str) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("password", password);

        self.http_client
            .post(&format!("{}/login", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute login request")
    }
    
    /// POST-Request the logout page of the auth service and return the response.
    /// 
    /// TODO: is this correct?
    pub async fn post_logout(&self) -> reqwest::Response {
        self.http_client
            .post(&format!("{}/logout", &self.address))
            .send()
            .await
            .expect("Failed to execute logout request")
    }

    /// POST-Request the auth service to verify a 2FA token and return the Response.
    /// 
    /// This route verifies a 2FA token.
    pub async fn post_verify_2fa(&self, email: &str, attempt_id: &str, code_2fa: &str) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("email", email);
        map.insert("loginAttemptId", attempt_id);
        map.insert("2FACode", code_2fa);

        self.http_client
            .post(&format!("{}/verify-2fa", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute verify-2fa request")
    }
    /// POST-Request the verify-token page of the auth service and return the response.
    /// 
    /// TODO: validate if this method is implemented correctly
    pub async fn post_verify_token(&self, session_token: &str) -> reqwest::Response {
        let mut map = HashMap::new();
        map.insert("token", session_token);

        self.http_client
            .post(&format!("{}/verify-token", &self.address))
            .json(&map)
            .send()
            .await
            .expect("Failed to execute verify-token request")
    }
}

/// Generate a random email address via Uuid
pub fn get_random_email() -> String {
    format!("{}@example.com", Uuid::new_v4())
}

/// Generate a random password via Uuid
pub fn get_random_password() -> String {
    format!("{}", Uuid::new_v4())
}