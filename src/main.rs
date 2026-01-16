use axum::{
    routing::get,
    Router,
    Json,
};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

// GET /health
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let listener = TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Server jalan di http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}