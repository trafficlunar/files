use std::path::PathBuf;

use axum::{extract::Multipart, http::StatusCode, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::{json, Value};
use tokio::fs;

pub async fn handler(mut multipart: Multipart) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set.");

    if let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let mut name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        let file_path = PathBuf::from("uploads").join(&name);

        let generate_filename = std::env::var("GENERATE_FILENAME")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

        if generate_filename {
            let random_name: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .map(char::from)
                .take(8)
                .collect();

            name = format!("{}.{}", random_name, name.split(".").last().unwrap());
        }

        if let Err(err) = fs::write(file_path, &data).await {
            tracing::error!("Failed to write file: {}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "success": false, "error": "Failed to write file" })),
            ));
        };

        tracing::info!("uploaded {} at a size of {} bytes", name, data.len());

        return Ok(Json(json!({
            "success": true,
            "name": name,
            "url": format!("{}/uploads/{}/raw", &base_url, &name),
            "url_preview": format!("{}/uploads/{}", &base_url, &name)
        })));
    }

    Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false }))))
}
