use axum::{body::Body, extract::Path, http::StatusCode, response::IntoResponse};
use tokio::fs::File;
use tokio_util::io::ReaderStream;

pub async fn route(Path(filename): Path<String>) -> impl IntoResponse {
    let file_path = format!("uploads/{}", filename);

    let file = match File::open(file_path).await {
        Ok(file) => file,
        Err(_) => return Err((StatusCode::NOT_FOUND, "file not found.")),
    };
    let stream = ReaderStream::new(file);
    Ok(Body::from_stream(stream))
}
