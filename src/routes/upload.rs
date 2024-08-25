use axum::{Json, http::StatusCode};
use serde_json::{json, Value};

pub async fn handler() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({ "success": true })))
}
