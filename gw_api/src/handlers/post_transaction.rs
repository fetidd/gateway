use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use eval_macro::eval;
use gw_core::{
    account::BankOneAccount, billing::Billing, merchant::Merchant, payment::Payment,
    transaction::transaction_builder::TransactionBuilder,
};
use tracing::instrument;
use validify::{Validate, Validify};

use crate::{
    app::AppState,
    error::{ErrorKind, GatewayError},
    requests::transaction::TransactionRequest,
    responses::transaction::TransactionResponse,
};

#[instrument]
pub async fn handle_post_transaction(
    State(app): State<AppState>,
    Json(mut payload): Json<TransactionRequest>,
) -> impl IntoResponse {
    let payment_data = match extract_payment_data(&mut payload) {
        Ok(p) => p,
        Err(e) => {
            return e.into_response();
        }
    };
    if let Err(e) = payment_data.validate().map_err(|e| GatewayError::from(e)) {
        return e.into_response();
    }
    let billing_data = match extract_billing_data(&mut payload) {
        Ok(p) => p,
        Err(e) => {
            return e.into_response();
        }
    };
    // let customer_data = extract_customer_data(&mut payload);
    // let merchant_id = payload.merchant_id;
    // get merchant record from database
    // let merchant_data = app.merchant_db.select(merchant_id);
    let merchant_data = Merchant::default();
    // get account record from database
    // let account_data = app.account_db.select(&payment_data);
    let account_data = Box::new(BankOneAccount {});
    let mut transaction = {
        let tb = TransactionBuilder::new()
            .transaction_type(payload.transaction_type)
            .amount(payload.amount)
            .payment(payment_data)
            .billing(billing_data)
            .merchant(merchant_data)
            .account(account_data);
        tb.build()
    };
    if let Err(validation_error) = transaction.validify() {
        return GatewayError {
            kind: ErrorKind::Validation,
            message: validation_error.to_string(),
        }
        .into_response();
    }
    let response = TransactionResponse::from(&transaction);
    (StatusCode::CREATED, Json(response)).into_response()
}

fn extract_payment_data(payload: &mut TransactionRequest) -> Result<Payment, GatewayError> {
    extract_trx_data(payload, TransactionRequest::take_payment_data)
}

fn extract_billing_data(payload: &mut TransactionRequest) -> Result<Billing, GatewayError> {
    extract_trx_data(payload, TransactionRequest::take_billing_data)
}

#[instrument]
fn extract_trx_data<T, R>(
    payload: &mut TransactionRequest,
    extract_fn: fn(&mut TransactionRequest) -> Option<R>,
) -> Result<T, GatewayError>
where
    R: TryInto<T>,
    <R as TryInto<T>>::Error: std::fmt::Display,
{
    if let Some(req) = extract_fn(payload) {
        match req.try_into() {
            Ok(obj) => Ok(obj),
            Err(e) => Err(GatewayError {
                kind: ErrorKind::Validation,
                message: e.to_string(),
            }),
        }
    } else {
        Err(GatewayError {
            kind: ErrorKind::Validation,
            message: String::from("missing  data"),
        })
    }
}

// eval! {
//     let fields = [("payment", "Payment"), ("billing", "Billing")]; // TODO create a function to do what python's .title() does
//     for (field, fieldTitle) in fields.iter() {
//         output! {
//             #[instrument]
//             fn extract_{{field}}_data(
//                 payload: &mut TransactionRequest,
//             ) -> Result<{{fieldTitle}}, GatewayError>
//             {
//                 if let Some(req) = payload.take_{{field}}_data() {
//                     match req.try_into() {
//                         Ok(obj) => Ok(obj),
//                         Err(e) => Err(GatewayError {
//                             kind: ErrorKind::Validation,
//                             message: e.to_string(),
//                         }),
//                     }
//                 } else {
//                     Err(GatewayError {
//                         kind: ErrorKind::Validation,
//                         message: String::from("missing {{field}} data"),
//                     })
//                 }
//             }
//         }
//     }
// }
