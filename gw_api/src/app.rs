use gw_core::repo::merchant::MerchantDb;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct AppStateInner {
    pub merchant_db: MerchantDb,
}

impl AppStateInner {
    pub async fn new(db_path: &str) -> AppStateInner {
        AppStateInner {
            merchant_db: MerchantDb::connect(db_path)
                .await
                .expect("failed to create app state"),
        }
    }
}

pub type AppState = Arc<Mutex<AppStateInner>>;

pub async fn create_appstate() -> AppState {
    let db_name = std::env::var("DATABASE_NAME").expect("DATABASE_NAME env variable not set");
    let db_path = format!("postgresql://localhost/{db_name}?user=gwuser&password=gwpass");
    Arc::new(Mutex::new(AppStateInner::new(&db_path).await))
}
