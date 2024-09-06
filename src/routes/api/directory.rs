use axum::Json;
use serde_json::{json, Value};
use walkdir::WalkDir;

// Handler for `/api/uploads`
pub async fn handler() -> Json<Value> {
    let uploads = WalkDir::new("uploads/");

    // Returns list of files
    let mut files: Vec<String> = uploads
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
        .collect();

    files.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase())); // Sorts files alphabetically case-insensitively

    // Send list of files as response
    Json(json!(files))
}
