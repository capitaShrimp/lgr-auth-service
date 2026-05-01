use axum::{Router, serve::Serve};
use std::error::Error;
use std::boxed::Box;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

// This struct encapsulates our application-related logic.
pub struct Application {
    server: Serve<TcpListener, Router, Router>,
    // address is exposed as a public field
    // so we have access to it in tests.
    pub address: String,
}

impl Application {
    // on success -- return a Result::Ok(Application{})
    // on error -- return a boxed Error trait object
    // -- this allows us to use '?' to propogate errors within the function body
    // -- these errors can be of any type which implements the Error trait
    pub async fn build(address: &str) -> Result<Self, Box<dyn Error>> {
        // Move the Router definition from `main.rs` to here.
        // Also, remove the `hello` route.
        // We don't need it at this point.

        let assets_dir = ServeDir::new("assets");
        let router = Router::new()
            .fallback_service(assets_dir);

        let listener = tokio::net::TcpListener::bind(address).await?;
        let address = listener.local_addr()?.to_string();
        let server = axum::serve(listener, router);

        // Create a new Application instance and return it
        return Result::Ok(Application{server, address})
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        println!("listening on {}", &self.address);
        self.server.await
    }
}