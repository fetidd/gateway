use axum::{routing::post, Router};
use gw_core::repo::{merchant::MerchantRepo, Pool};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::post_transaction::handle_post_transaction;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/transaction", post(handle_post_transaction))
        .with_state(app_state)
}

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub merchants: MerchantRepo,
}

impl AppStateInner {
    pub async fn new(pool: Box<Pool>) -> AppStateInner {
        AppStateInner {
            merchants: MerchantRepo { pool },
        }
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;

pub async fn create_appstate(pool: Pool) -> AppState {
    Arc::new(Mutex::new(AppStateInner::new(Box::new(pool)).await))
}
