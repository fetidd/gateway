use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
// use eval_macro::eval;
use gw_core::{
    account::AcquirerAccount,
    billing::Billing,
    currency::Currency,
    merchant::Merchant,
    payment::Payment,
    repo::Repo,
    transaction::{transaction_builder::TransactionBuilder, Transaction},
};
use tokio::sync::Mutex;
use tracing::instrument;
use validify::{Validate, Validify};

use crate::{
    app::{AppState, AppStateInner},
    error::{ErrorKind, GatewayError},
    requests::transaction::TransactionRequest,
    responses::transaction::TransactionResponse,
};

#[instrument]
pub async fn handle_post_transaction(
    State(app): State<AppState>,
    Json(mut payload): Json<TransactionRequest>,
) -> Result<impl IntoResponse, GatewayError> {
    let payment = extract_payment_data(&mut payload)?;
    payment.validate()?;
    let billing = extract_billing_data(&mut payload)?;
    // let customer = extract_customer_data(&mut payload)?;
    let merchant_id = payload.merchant_id;
    let merchant = Merchant::find(&merchant_id, &app.pool).await?;
    let account =
        AcquirerAccount::find(&merchant_id, &payment, payload.currency, &app.pool).await?;
    let mut transaction = {
        let tb = TransactionBuilder::new()
            .transaction_type(payload.transaction_type)
            .amount(payload.amount)
            .currency(payload.currency)
            .payment(payment)
            .billing(billing)
            .merchant(merchant)
            .account(account);
        tb.build()
    };
    transaction.validify()?;
    // send the transaction off to acquirers etc.
    transaction.save(&app.pool)?;
    let response = TransactionResponse::from(&transaction);
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

async fn find_merchant(
    app: &Arc<Mutex<AppStateInner>>,
    id: &String,
) -> Result<Merchant, GatewayError> {
    let app_access = app.lock().await;
    let merchant_data = app_access
        .merchants
        .find(id)
        .await
        .map_err(|_| GatewayError {
            kind: ErrorKind::Resource,
            message: format!("merchant {id} does not exist"),
        })?;
    Ok(merchant_data)
}

async fn find_account(
    app: &Arc<Mutex<AppStateInner>>,
    merchant_id: &String,
    payment_data: &Payment,
    currency: Currency,
) -> Result<AcquirerAccount, GatewayError> {
    let app_access = app.lock().await;
    let account_data = app_access
        .accounts
        .select_for(&merchant_id, &payment_data, currency)
        .await?;
    Ok(account_data)
}

fn extract_payment_data(payload: &mut TransactionRequest) -> Result<Payment, GatewayError> {
    extract_trx_data(payload, TransactionRequest::take_payment_data, "payment")
}

fn extract_billing_data(payload: &mut TransactionRequest) -> Result<Billing, GatewayError> {
    extract_trx_data(payload, TransactionRequest::take_billing_data, "billing")
}

#[instrument]
fn extract_trx_data<T, R>(
    payload: &mut TransactionRequest,
    extract_fn: fn(&mut TransactionRequest) -> Option<R>,
    field: &str,
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
            message: format!("missing {field} data"),
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

#[cfg(test)]
mod tests {}
