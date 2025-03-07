use axum::{routing::post, Router};
use dotenvy::dotenv;
use gw_api::{app::create_appstate, handlers::post_transaction::handle_post_transaction};

#[tokio::main]
async fn main() {
    dotenv().expect("no .env file!");
    tracing_subscriber::fmt::init();
    let app_state = create_appstate().await;
    let app = Router::new()
        .route("/transaction", post(handle_post_transaction))
        .with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
