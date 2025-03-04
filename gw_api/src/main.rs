use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use gw_api::requests;
use requests::transaction::TransactionRequest;
use serde_json::json;
use tokio::sync::Mutex;
use tracing::instrument;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app_state = Arc::new(Mutex::new(AppState {}));
    let app = Router::new()
        .route("/transaction", post(handle_post_transaction))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

struct AppState {}

#[instrument]
async fn handle_post_transaction(Json(payload): Json<TransactionRequest>) -> impl IntoResponse {
    (StatusCode::IM_A_TEAPOT, Json(json!({"name": "derp"})))
}
