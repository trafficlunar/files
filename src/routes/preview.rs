use std::{fs, os::unix::fs::MetadataExt, path::PathBuf};

use askama::Template;
use axum::{extract::Path, http::StatusCode, response::Html};
use chrono::{DateTime, Local};

#[derive(Template)]
#[template(path = "preview.html")]
struct PreviewTemplate<'a> {
    file: &'a str,
    file_modified: &'a str,
    file_size: &'a str,
    page_title: &'a str,
}

#[derive(Template)]
#[template(path = "error.html")]
struct ErrorTemplate<'a> {
    file: &'a str,
    page_title: &'a str,
    error: &'a str,
}

pub async fn handler(
    Path(filename): Path<String>,
) -> Result<Html<String>, (StatusCode, Html<String>)> {
    let page_title = std::env::var("PAGE_TITLE").unwrap_or_else(|_| "files".to_string());

    let file_path = PathBuf::from("uploads").join(&filename);
    let metadata = fs::metadata(file_path).map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            render_error(&filename, &page_title, "File not found"),
        )
    })?;

    let modified_time = metadata
        .modified()
        .map(|time| {
            DateTime::<Local>::from(time)
                .format("%Y-%m-%d at %H:%M:%S")
                .to_string()
        })
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                render_error(&filename, &page_title, "Internal server error while getting the modified time"),
            )
        })?;

    const SIZES: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let size = metadata.size();
    let size_index = (size as f64).log(1000.0).floor() as usize;
    let size_divided = size as f64 / 1000_f64.powi(size_index as i32);
    let size_formatted = format!("{:.1} {}", size_divided, SIZES[size_index]);

    let template = PreviewTemplate {
        file: &filename,
        file_modified: &modified_time,
        file_size: &size_formatted,
        page_title: &page_title,
    };

    template.render().map(Html).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            render_error(&filename, &page_title, "Error rendering template"),
        )
    })
}

fn render_error(filename: &str, page_title: &str, error: &str) -> Html<String> {
    let error_template = ErrorTemplate {
        file: filename,
        page_title,
        error,
    };

    error_template
        .render()
        .map(Html)
        .unwrap_or_else(|_| Html("Error rendering error template".to_string()))
}
