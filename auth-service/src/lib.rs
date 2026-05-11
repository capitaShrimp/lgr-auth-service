use axum::{routing::{post}, Router, serve::Serve};
use std::error::Error;
use std::boxed::Box;
use tokio::net::TcpListener;
use tower_http::services::{ServeDir};


pub mod app_state;
pub mod domain;
pub mod routes;
pub mod services;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    /// Build a new instance of the applicaiton (authentication service).
    pub async fn build(app_state: app_state::AppState, address: &str) -> Result<Self, Box<dyn Error>> {
        let router = Router::new()
            // connect routes to handlers
            .fallback_service(ServeDir::new("assets"))
            .route("/signup", post(routes::signup_handler))
            .route("/login", post(routes::login_handler))
            .route("/logout", post(routes::logout_handler))
            .route("/verify-2fa", post(routes::verify_2fa_handler))
            .route("/verify-token", post(routes::verify_token_handler))
            .with_state(app_state); // attach the application state to the router -- all routes have access to app_state

        // on success -- return a Result::Ok(Application{})
        // on error -- return a boxed Error trait object
        // -- this allows us to use '?' to propogate errors within the function body
        // -- propogated errors can be of any type which implements the Error trait
        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // create a new auth-service applicaiton instance and return it
        return Result::Ok(Application{server, address})
    }

    /// Start the application (authentication service) server.
    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}