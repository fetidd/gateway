use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use gw_api::{requests, responses::transaction::TransactionResponse};
use gw_core::transaction::transaction_builder::TransactionBuilder;
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

#[derive(Debug)]
struct AppState {}

#[instrument]
async fn handle_post_transaction(
    State(app): State<AppState>,
    Json(payload): Json<TransactionRequest>,
) -> impl IntoResponse {
    let merchant_id = payload.merchant_id;
    // get merchant record from database
    // let merchant_data = app.merchant_db.select(merchant_id);
    let transaction = {
        let tb = TransactionBuilder::new()
            .transaction_type(payload.transaction_type)
            .amount(payload.amount);
        if let Some(payment) = payload.payment {
            let tb = tb.payment(payment.into());
        } else {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "missing payment"})),
            );
        }
    };

    (StatusCode::IM_A_TEAPOT, Json(json!({"name": "derp"})))
}
