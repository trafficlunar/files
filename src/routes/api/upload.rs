use std::path::PathBuf;

use axum::{
    extract::{rejection::JsonRejection, Multipart},
    http::StatusCode,
    Json,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio::fs;

#[derive(Deserialize)]
pub struct UploadSettings {
    generate_name: bool,
}

pub async fn handler(
    result: Result<Json<UploadSettings>, JsonRejection>,
    mut multipart: Multipart,
) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
    let payload = match result {
        Ok(Json(payload)) => payload,
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(json!({ "success": false, "error": "Invalid request body" })),
            ))
        }
    };

    let base_url = std::env::var("BASE_URL").expect("BASE_URL must be set.");
    let generate_filename = payload.generate_name
        || std::env::var("GENERATE_FILENAME")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .unwrap_or(false);

    if let Some(field) = multipart.next_field().await.unwrap_or(None) {
        let mut name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        if generate_filename {
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
            "url": format!("{}/uploads/{}/raw", &base_url, &name),
            "url_preview": format!("{}/uploads/{}", &base_url, &name)
        })));
    }

    Err((StatusCode::BAD_REQUEST, Json(json!({ "success": false }))))
}
