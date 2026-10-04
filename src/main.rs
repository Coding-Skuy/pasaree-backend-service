mod katalog;
mod lapak;
mod transaksi;

use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn kesehatan() -> Json<Value> {
    Json(json!({"layanan": "pasaree-backend-service", "status": "baik", "db": "pasaree"}))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/kesehatan", get(kesehatan))
        .nest("/katalog", katalog::rute())
        .nest("/lapak", lapak::rute())
        .nest("/transaksi", transaksi::rute());
    let pendengar = tokio::net::TcpListener::bind("0.0.0.0:8101").await.unwrap();
    axum::serve(pendengar, app).await.unwrap();
}
