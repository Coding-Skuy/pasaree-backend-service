use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub fn rute() -> Router {
    Router::new().route("/", get(daftar))
}

async fn daftar() -> Json<Value> {
    Json(json!({"katalog": [], "catatan": "ID ULID string, harga integer IDR, paginasi cursor"}))
}
