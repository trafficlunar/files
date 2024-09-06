use std::path::PathBuf;

use axum::{extract::rejection::JsonRejection, http::StatusCode, Json};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs;

#[derive(Deserialize)]
pub struct RenameFile {
    name: String,
    new_name: String,
}

// Handler for `/api/rename`
pub async fn handler(
    result: Result<Json<RenameFile>, JsonRejection>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Checks that JSON body is there / correct
    match result {
        Ok(Json(payload)) => {
            // Get file paths
            let uploads = PathBuf::from("uploads");
            let file = uploads.join(&payload.name);
            let new_file = uploads.join(&payload.new_name);

            // Rename file
            match fs::rename(file, new_file).await {
                Ok(_) => Ok(Json(json!({ "success": true }))),
                Err(_) => Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "success": false, "error": "Failed to rename file" })),
                )),
            }
        }
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "error": "Invalid request body" })),
        )),
    }
}
