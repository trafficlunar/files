use std::path::PathBuf;

use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs;

#[derive(Deserialize)]
pub struct DeleteFile {
    name: String,
}

// Handler for `/api/delete`
pub async fn handler(
    result: Result<Json<DeleteFile>, JsonRejection>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Checks that JSON body is there / correct
    match result {
        Ok(Json(payload)) => {
            // Get file
            let file = PathBuf::from("uploads").join(&payload.name);

            // Delete file
            match fs::remove_file(file).await {
                Ok(_) => Ok(Json(json!({ "success": true }))),
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": "Failed to delete file" })),
                )),
            }
        }
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Invalid request body" })),
        )),
    }
}
