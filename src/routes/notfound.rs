use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

pub async fn handler() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": "404 - not found"})))
}