use std::fs;

use askama::Template;
use axum::{extract::Path, http::StatusCode, response::Html};
use chrono::{DateTime, Local};

#[derive(Template)]
#[template(path = "preview.html")]
struct PreviewTemplate<'a> {
    file: &'a str,
    file_modified: &'a str,
    page_title: &'a str,
}

pub async fn route(Path(filename): Path<String>) -> Result<Html<String>, StatusCode> {
    let page_title = match std::env::var("PAGE_TITLE") {
        Ok(page_title) => page_title,
        Err(_) => "files".to_string()
    };

    let metadata = match fs::metadata(format!("uploads/{}", filename)) {
        Ok(metadata) => metadata,
        Err(_) => return Err(StatusCode::NOT_FOUND),
    };
    let modified_time = match metadata.modified() {
        Ok(time) => DateTime::<Local>::from(time).format("%Y-%m-%d at %H:%M:%S").to_string(),
        Err(_) => "unknown".to_string(),
    };

    let template = PreviewTemplate {
        file: filename.as_str(),
        file_modified: modified_time.as_str(),
        page_title: page_title.as_str(),
    };
    let rendered = template.render().unwrap();
    Ok(Html(rendered))
}
