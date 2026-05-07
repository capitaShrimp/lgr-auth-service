use auth_service::Application;

#[tokio::main]
async fn main() {

    let address: &str = "0.0.0.0:3001";
    // let address = "127.0.0.1:3000";

    let app = Application::build(address)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to run app");
}