use std::path::PathBuf;

use axum::{extract::Multipart, http::StatusCode, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::{json, Value};
use tokio::fs;

// Handler for `/api/upload`
pub async fn handler(mut multipart: Multipart) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    // Get .env variables
    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set.");

    // Check that file is in request
    if let Some(field) = multipart.next_field().await.unwrap_or(None) {
        // Get the file name from the request (not the custom name that a request can have)
        let mut name = field.file_name().unwrap().to_string();
        // Get data of file
        let data = field.bytes().await.unwrap();
        // Find path for the file
        let file_path = PathBuf::from("uploads").join(&name);

        // Get .env variables
        let generate_filename = std::env::var("GENERATE_FILENAME")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);
        let generate_filename_length: usize = std::env::var("GENERATE_FILENAME_LENGTH")
            .unwrap_or_else(|_| "8".to_string())
            .parse()
            .unwrap_or(8);

        if generate_filename {
            let random_name: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .map(char::from)
                .take(generate_filename_length)
                .collect();

            // Format name as the generated name and its file extension
            name = format!("{}.{}", random_name, name.split(".").last().unwrap());
        }

        // Write file to the path using the data
        if let Err(err) = fs::write(file_path, &data).await {
            tracing::error!("Failed to write file: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "error": "Failed to write file" })),
            ));
        };

        tracing::info!("uploaded {} at a size of {} bytes", name, data.len());

        // Send response
        return Ok(Json(json!({
            "success": true,
            "name": name,
            "url": format!("{}/uploads/{}/raw", &base_url, &name),
            "url_preview": format!("{}/uploads/{}", &base_url, &name)
        })));
    }

    Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false }))))
}
