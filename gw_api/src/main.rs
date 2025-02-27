mod requests;
mod responses;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use requests::transaction::TransactionRequest;
use responses::{payment::PaymentResponse, transaction::TransactionResponse};
use tracing::{error, instrument};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/transaction", post(handle_post_transaction));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

#[instrument]
async fn handle_post_transaction(
    Json(payload): Json<TransactionRequest>,
) -> (StatusCode, Json<TransactionResponse>) {
    let payment = match payload.payment {
        Some(p) => p,
        None => {
            error!("Missing payment details in transaction");
            return (StatusCode::BAD_REQUEST, Json(TransactionResponse {result: "failed".into(), ..Default::default()}));
        }
    };
    let res = TransactionResponse {
        baseamount: Some(payload.baseamount),
        result: String::from("success"),
        payment: Some(PaymentResponse {
            payment_type: payment.payment_type,
            account_number: payment.account_number.clone(),
        }),
        ..Default::default()
    };
    (StatusCode::CREATED, Json(res))
}
