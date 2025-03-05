use std::sync::Arc;

use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppStateInner {}

impl AppStateInner {
    pub fn new() -> AppStateInner {
        AppStateInner {}
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;

pub fn create_appstate() -> AppState {
    Arc::new(Mutex::new(AppStateInner::new()))
}
