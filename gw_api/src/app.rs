use axum::{routing::post, Router};
use gw_core::repo::merchant::MerchantRepo;
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
    pub merchant_repo: MerchantRepo,
}

impl AppStateInner {
    pub async fn new(db_path: &str) -> AppStateInner {
        AppStateInner {
            merchant_repo: MerchantRepo::connect(db_path)
                .await
                .expect("failed to create app state"),
        }
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;

pub async fn create_appstate() -> AppState {
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");
    Arc::new(Mutex::new(AppStateInner::new(&db_url).await))
}
