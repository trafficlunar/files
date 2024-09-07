use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

use askama::Template;
use axum::{extract::Path, http::StatusCode, response::Html};
use chrono::{DateTime, Local, Utc};

#[path = "../error.rs"]
mod error;

#[derive(Template)]
#[template(path = "preview.html")]
struct PreviewTemplate<'a> {
    file: &'a str,
    file_modified: &'a str,
    file_size: &'a str,
    mime_type: &'a str,
    page_title: &'a str,
}

// Handler for `/:filename`
pub async fn handler(
    Path(filename): Path<String>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    // Get .env variables
    let page_title = std::env::var("PAGE_TITLE").unwrap_or_else(|_| "files".to_string());

    // Get file path, url, and metadata
    let file_path = PathBuf::from("uploads").join(&filename);
    let formatted_url = format!("/uploads/{}", &filename);
    let metadata = fs::metadata(&file_path).map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            error::render_error(&formatted_url, "File not found"),
        )
    })?;

    // Get file modified time in human readable text
    let modified_time = metadata
        .modified()
        .map(|time| {
            DateTime::<Local>::from(time)
                .with_timezone(&Utc)
                .format("%Y-%m-%d at %H:%M:%S UTC")
                .to_string()
        })
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                error::render_error(
                    &formatted_url,
                    "Error occurred while getting the modified time",
                ),
            )
        })?;

    // Convert the size (bytes) into a rounded format
    const SIZES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let size = metadata.size();
    let size_index = (size as f64).log(1000.0).floor() as usize;
    let size_divided = size as f64 / 1000_f64.powi(size_index as i32);
    let size_formatted = format!("{:.1} {}", size_divided, SIZES[size_index]);

    // Guess mime type of file
    let mime_type = match mime_guess::from_path(&file_path).first_raw() {
        Some(mime) => mime,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                error::render_error(&formatted_url, "Error occurred while getting the MIME type"),
            ))
        }
    };

    // Render template
    let template = PreviewTemplate {
        file: &filename,
        file_modified: &modified_time,
        file_size: &size_formatted,
        mime_type,
        page_title: &page_title,
    };

    template.render().map(Html).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            error::render_error(&formatted_url, "Error rendering template"),
        )
    })
}
