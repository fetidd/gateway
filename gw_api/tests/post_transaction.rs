mod common;
use common::{create_request, create_server, CreateRequestAction};
use serde_json::json;
use sqlx;

macro_rules! test_case {
    ($name:ident, $endpoint:expr, $status_code:expr, $body:expr, $overrides:expr) => {
        #[sqlx::test(migrations = "../gw_core/migrations")]
        async fn $name(_pool: sqlx::PgPool) {
            let server = create_server(_pool.clone());
            let overrides = $overrides;
            let response = server
                .post($endpoint)
                .json(&create_request(overrides))
                .await;
            assert_eq!(response.status_code(), $status_code);
            assert_eq!(response.json::<serde_json::Value>(), $body);
        }
    };
    ($name:ident, $endpoint:expr, $status_code:expr, $body:expr) => {
        #[sqlx::test(migrations = "../gw_core/migrations")]
        async fn $name(_pool: sqlx::PgPool) {
            let server = create_server(_pool.clone());
            let response = server
                .post($endpoint)
                .json(&create_request(Vec::<CreateRequestAction>::new()))
                .await;
            assert_eq!(response.status_code(), $status_code);
            assert_eq!(response.json::<serde_json::Value>(), $body);
        }
    };
}

test_case! {simple_transaction, "/transaction", 201, json!({
    "amount": 12345,
    "currency": "GBP",
    "payment": {
        "scheme": "VISA",
        "pan": "400011######3333",
        "expiry_year": 2026,
        "expiry_month": 12,
        "type": "CARD"
    },
    "billing": {
        "country": "GB"
    },
    "status": "SUCCESS"
})}

test_case! {merchant_doesnt_exist, "/transaction", 404, json!({
    "error": "RESOURCE",
    "message": "merchant invalid123 does not exist"
}), vec![("merchant_id", "invalid123").into()]}

test_case! {missing_payment_details, "/transaction", 400, json!({
    "error": "VALIDATION",
    "message": "missing payment data"
}), vec!["!payment".into()]}

test_case! {missing_pan, "/transaction", 400, json!({
    "error": "VALIDATION",
    "message": "missing fields: pan"
}), vec!["!payment.pan".into()]}

test_case! {bad_pan, "/transaction", 400, json!({
    "error": "VALIDATION",
    "message": "invalid pan length"
}), vec![("payment.pan", "400011112222333344445555").into()]}

test_case! {missing_pan_and_security_code, "/transaction", 400, json!({
    "error": "VALIDATION",
    "message": "missing fields: pan, security_code"
}), vec!["!payment.pan".into(), "!payment.security_code".into()]}

test_case! {no_account, "/transaction", 404, json!({
    "error": "RESOURCE",
    "message": "no account found"
}), vec![("currency", "JPY").into()]}

test_case! {no_account_or_country_but_country_errors, "/transaction", 400, json!({
    "error": "VALIDATION",
    "message": "TypeError:  is not a recognised country code"
}), vec![("currency", "JPY").into(), "!billing.country".into()]}
