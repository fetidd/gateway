use dotenvy::dotenv;
use gw_api::app::{create_appstate, create_router};
use gw_core::repo::Pool;

#[tokio::main]
async fn main() {
    dotenv().expect("no .env file!");
    tracing_subscriber::fmt::init();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL env variable not set");
    let pool = Pool::new(&db_url)
        .await
        .expect("failed to create database pool");
    let app_state = create_appstate(pool);
    let app = create_router(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
