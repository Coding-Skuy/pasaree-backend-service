use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

pub fn rute() -> Router {
    Router::new().route("/", get(daftar))
}

async fn daftar() -> Json<Value> {
    Json(json!({"lapak": [], "catatan": "wajib titik_serah_lumbung_id, kurasi sebelum tayang"}))
}
