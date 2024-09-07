use std::path::PathBuf;

use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::IntoResponse,
};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

// Handler for `/:filename/raw`
pub async fn handler(Path(filename): Path<String>) -> impl IntoResponse {
    // Get file path
    let file_path = PathBuf::from("uploads").join(&filename);

    // Open file
    let file = match File::open(&file_path).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, "File not found")),
    };

    // Guess mime type of file
    let content_type = match mime_guess::from_path(&file_path).first_raw() {
        Some(mime) => mime,
        None => "text/plain"
    };

    // Send file as response
    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);
    let headers = [(header::CONTENT_TYPE, content_type)];

    Ok((headers, body))
}
