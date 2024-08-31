use askama::Template;
use axum::{http::StatusCode, response::Html};
use walkdir::WalkDir;

#[path = "../error.rs"]
mod error;

#[derive(Template)]
#[template(path = "directory.html")]
struct DirectoryTemplate<'a> {
    files: Vec<String>,
    page_title: &'a str,
}

pub async fn handler() -> Result<Html<String>, (StatusCode, Html<String>)> {
    let page_title = std::env::var("PAGE_TITLE").unwrap_or_else(|_| "files".to_string());

    let uploads = WalkDir::new("uploads/");
    let files: Vec<String> = uploads
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
        .collect();

    let template = DirectoryTemplate {
        files,
        page_title: &page_title,
    };

    template.render().map(Html).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            error::render_error("/uploads", "Error rendering template"),
        )
    })
}
