use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use gw_api::{
    error::{ErrorKind::Validation, GatewayError},
    requests::{self, transaction::payment::PaymentRequest},
    responses::transaction::TransactionResponse,
};
use gw_core::{payment::Payment, transaction::transaction_builder::TransactionBuilder};
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

#[derive(Debug, Clone)]
struct AppState {}

#[instrument]
async fn handle_post_transaction(
    State(app): State<Arc<Mutex<AppState>>>,
    Json(mut payload): Json<TransactionRequest>,
) -> impl IntoResponse {
    let payment_data = extract_payment_data(&mut payload);
    let merchant_id = payload.merchant_id;
    // get merchant record from database
    // let merchant_data = app.merchant_db.select(merchant_id);
    let transaction = {
        let tb = TransactionBuilder::new()
            .transaction_type(payload.transaction_type)
            .amount(payload.amount);
    };

    (StatusCode::IM_A_TEAPOT, Json(json!({"name": "derp"})))
}

fn extract_payment_data(payload: &mut TransactionRequest) -> Result<Payment, GatewayError> {
    extract_transaction_data(
        payload,
        TransactionRequest::take_payment_data,
        "missing payment data",
    )
}

fn extract_transaction_data<T, R>(
    payload: &mut TransactionRequest,
    extractor: fn(&mut TransactionRequest) -> Option<R>,
    missing_err: &str,
) -> Result<T, GatewayError>
where
    T: TryFrom<R>,
    T::Error: std::fmt::Display,
{
    if let Some(payment_req) = extractor(payload) {
        match payment_req.try_into() {
            Ok(payment) => Ok(payment),
            Err(e) => Err(GatewayError {
                kind: Validation,
                message: e.to_string(),
            }),
        }
    } else {
        Err(GatewayError {
            kind: Validation,
            message: missing_err.to_string(),
        })
    }
}
