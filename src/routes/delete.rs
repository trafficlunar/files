use std::path::PathBuf;

use axum::{Json, http::StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs;

#[derive(Deserialize)]
pub struct DeleteFile {
    name: String,
}

pub async fn handler(Json(payload): Json<DeleteFile>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let file = PathBuf::from("uploads").join(&payload.name);
    
    match fs::remove_file(file).await {
        Ok(_) => Ok(Json(json!({ "success": true }))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "success": false, "error": "Failed to delete file" })))),
    }
}
