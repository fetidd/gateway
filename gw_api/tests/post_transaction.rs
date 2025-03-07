use axum_test::TestServer;
use gw_api::app::{create_appstate, create_router, AppStateInner};
use gw_core::repo::merchant::MerchantRepo;

#[sqlx::test]
async fn simple_happy_transaction(pool: PgPool) -> sqlx::Result<()> {
    // let merchant_repo = MerchantRepo { pool: PgPool }};
    // let app_state = AppStateInner { merchant_repo: todo!() }};
    // let router = create_router(app_state);
    // let server = TestServer::new(router).expect("failed to create test router");
}
