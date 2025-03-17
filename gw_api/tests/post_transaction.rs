use axum_test::TestServer;
use gw_api::app::{create_appstate, create_router};
use gw_core::repo::Pool;
use serde_json::json;
use sqlx;

#[sqlx::test(migrations = "../gw_core/migrations")]
async fn simple_transaction(_pool: sqlx::PgPool) {
    let server = create_server(_pool.clone());
    let response = server
        .post("/transaction")
        .json(&json!({
            "amount": 12345,
            "currency": "GBP",
            "transaction_type": "Auth",
            "merchant_id": "merchant123",
            "payment": {
                "scheme": "VISA",
                "pan": "4000111122223333",
                "security_code": "123",
                "expiry_year": 2026,
                "expiry_month": 12,
                "payment_type": "CARD"
            },
            "billing": {
                "country": "GB"
            }
        }))
        .await;
    assert_eq!(response.status_code(), 201);
    assert_eq!(
        response.json::<serde_json::Value>(),
        json!({
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
        })
    );
}

fn create_server(pool: sqlx::PgPool) -> TestServer {
    let pool = Pool::from(pool);
    let app_state = create_appstate(pool);
    let router = create_router(app_state);
    let server = TestServer::new(router).expect("creating server failed");
    server
}
