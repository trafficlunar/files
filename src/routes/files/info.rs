use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

use axum::{extract::Path, http::StatusCode, Json};
use chrono::{DateTime, Local};
use serde_json::{json, Value};

// Handler for `/:filename/info`
pub async fn handler(
    Path(filename): Path<String>,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Get .env variables
    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set");

    // Get file path and metadata
    let file_path = PathBuf::from("uploads").join(&filename);
    let metadata = fs::metadata(file_path).map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({ "success": false, "error": "File not found" })),
        )
    })?;

    // Get file modified time in Unix
    let modified_time = metadata
        .modified()
        .map(|time| DateTime::<Local>::from(time).timestamp())
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "error": "Error occurred while getting the modified time" })),
            )
        })?;

    // Send response
    Ok(Json(json!({
        "success": true,
        "file": &filename,
        "modified": modified_time,
        "size": metadata.size(),
        "url": format!("{}/uploads/{}/raw", &base_url, &filename),
        "url_preview": format!("{}/uploads/{}", &base_url, &filename)
    })))
}
