mod requests;
mod responses;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use requests::transaction::TransactionRequest;
use serde_json::json;
use tracing::instrument;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/transaction", post(handle_post_transaction));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[instrument]
async fn handle_post_transaction(
    Json(payload): Json<TransactionRequest>,
) -> impl IntoResponse {
    (StatusCode::CREATED, Json(json!({"name": "derp"})))
}
