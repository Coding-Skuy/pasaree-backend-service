use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub fn rute() -> Router {
    Router::new().route("/", get(daftar))
}

async fn daftar() -> Json<Value> {
    Json(json!({"transaksi": [], "catatan": "checkout multi-lapak, bayar idempoten, escrow lalu payout via Lumbung"}))
}
