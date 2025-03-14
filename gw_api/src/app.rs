use axum::{routing::post, Router};
use gw_core::repo::{account::AccountRepo, merchant::MerchantRepo, Pool};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::handlers::post_transaction::handle_post_transaction;

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/transaction", post(handle_post_transaction))
        .with_state(app_state)
}

#[derive(Debug)]
pub struct AppStateInner {
    pub merchants: MerchantRepo,
    pub accounts: AccountRepo,
}

impl AppStateInner {
    pub async fn new(pool: Arc<Pool>) -> AppStateInner {
        AppStateInner {
            merchants: MerchantRepo {
                pool: Arc::clone(&pool),
            },
            accounts: AccountRepo {
                pool: Arc::clone(&pool),
            },
        }
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;

pub async fn create_appstate(pool: Pool) -> AppState {
    Arc::new(Mutex::new(AppStateInner::new(Arc::new(pool)).await))
}
