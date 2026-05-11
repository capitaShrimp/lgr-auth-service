use auth_service::Application;
use auth_service::app_state::{AppState, UserStoreType};
use auth_service::services::hashmap_user_store::{HashMapUserStore};

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {

    let address: &str = "0.0.0.0:3001";
    // let address = "127.0.0.1:3001";

    let user_store: UserStoreType = Arc::new(RwLock::new(HashMapUserStore::default()));
    let app_state = AppState::new(user_store);
    
    // create a new application instance and start the server
    let app = Application::build(app_state, address)
        .await
        .expect("Failed to build app. Likely port collision.");

    app.run().await.expect("Failed to run app.");
}