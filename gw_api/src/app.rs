use std::sync::Arc;

use axum::{routing::post, Router};
use gw_core::pool::Pool;

use crate::handlers::post_transaction::handle_post_transaction;

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/transaction", post(handle_post_transaction))
        .with_state(app_state)
}

#[derive(Debug)]
pub struct AppState {
    pub pool: Pool,
}

impl AppState {
    pub fn new(pool: Pool) -> AppState {
        AppState { pool }
    }
}
