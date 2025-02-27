mod requests;
mod responses;

use axum::{http::StatusCode, routing::post, Json, Router};
use gw_core::{payment_type::PaymentType, utils};
use requests::transaction::TransactionRequest;
use responses::transaction::TransactionResponse;
use tracing::{error, instrument};

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
) -> (StatusCode, Json<TransactionResponse>) {
    let payment = match payload.payment {
        Some(p) => p,
        None => {
            error!("Missing payment details in transaction");
            return (
                StatusCode::BAD_REQUEST,
                Json(TransactionResponse {
                    result: "failed".into(),
                    ..Default::default()
                }),
            );
        }
    };
    let masker = match payment.payment_type {
        PaymentType::Card { .. } => utils::mask_pan,
        PaymentType::Account => utils::mask_account_number,
    };
    let res = TransactionResponse {
        baseamount: Some(payload.baseamount),
        result: String::from("success"),
        // payment: Some(PaymentResponse {
        //     payment_type: payment.payment_type,
        //     account_number: masker(&payment.account_number),
        // }),
        ..Default::default()
    };
    (StatusCode::CREATED, Json(res))
}
