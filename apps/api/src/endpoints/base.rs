use std::collections::HashMap;

use acme::timestamp;
use axum;
use serde_json::{json, Value};

pub type Dictionary<T> = HashMap<String, T>;
pub type Container<T> = Dictionary<Vec<T>>;

pub fn create_route() -> axum::Router {
    axum::Router::new()
        .route("/", axum::routing::get(base))
}

pub async fn base() -> axum::Json<Value> {
    let mut cache: Dictionary<String> = Dictionary::new();
    let timestamp = timestamp();
    cache.insert(String::from("timestamp"), timestamp.to_string());
    axum::Json(
        json!(cache)
    )
}

pub async fn fetch(key: String, value: String) -> axum::Json<Value> {
    let mut cache: Dictionary<String> = Dictionary::new();
    cache.insert(key, value);
    axum::Json(
        json!(cache)
    )
}

pub async fn store(key: String, value: String) -> axum::Json<Value> {
    let mut cache: Dictionary<String> = Dictionary::new();
    cache.insert(key, value);
    axum::Json(
        json!(cache)
    )
}