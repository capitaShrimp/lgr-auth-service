use std::sync::Arc;
use tokio::sync::RwLock;

use crate::services::hashmap_user_store::HashMapUserStore;

// type alias to improve readability
// - Arc allow shared ownership in a thread-safe way
// -- clones the Arc smart pointer instead of the underlying, heap-allocated data
// - RwLock is a reader-writer lock for interior mutability
// -- safe data mutation across threads
pub type UserStoreType = Arc<RwLock<HashMapUserStore>>;

/// Basic struct which holds all the data we want to share across route handlers.
#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}

impl AppState {
    pub fn new(user_store: UserStoreType) -> Self {
        Self { user_store }
    }
}