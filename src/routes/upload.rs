use std::path::PathBuf;

use axum::{extract::Multipart, http::StatusCode, Json};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde_json::{json, Value};
use tokio::fs;

pub async fn handler(mut multipart: Multipart) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set.");
    let generate_filename =
        std::env::var("GENERATE_FILENAME").unwrap_or_else(|_| "false".to_string());

    if let Some(field) = multipart.next_field().await.unwrap() {
        let mut name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if generate_filename == "true" {
            let random_name: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .map(char::from)
                .take(8)
                .collect();

            name = format!("{}.{}", random_name, name.split(".").last().unwrap());
        }

        let file_path = PathBuf::from("uploads").join(&name);

        fs::write(file_path, &data).await.unwrap();
        tracing::info!("uploaded {} at a size of {} bytes", name, data.len());

        return Ok(Json(json!({
            "success": true,
            "name": name,
            "url": format!("{}{}/raw", &base_url, &name),
            "url_preview": base_url + &name
        })));
    }

    Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false }))))
}
