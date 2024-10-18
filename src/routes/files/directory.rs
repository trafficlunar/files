use askama::Template;
use axum::{http::StatusCode, response::Html, Form};
use serde::Deserialize;
use walkdir::WalkDir;

use crate::password;

#[path = "../error.rs"]
mod error;

#[derive(Template)]
#[template(path = "directory.html")]
struct DirectoryTemplate<'a> {
    files: Vec<String>,
    password: &'a str,
    enable_actions: bool,
    page_title: &'a str,
}

#[derive(Template)]
#[template(path = "login.html")]
struct LoginTemplate<'a> {
    page_title: &'a str
}

#[derive(Deserialize)]
pub struct LoginForm {
    password: String
}

// Handler for `/`
// Handler for login
pub async fn handler() -> Result<Html<String>, (StatusCode, Html<String>)> {
    // Get .env variables
    let protect_directory = std::env::var("PROTECT_DIRECTORY")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    // If there is no password or PROTECT_DIRECTORY is false - render the uploads template
    if password::get_password() == "" || !protect_directory {
        return render_upload_template();
    } else {
        // Show login page if there is a password
        // Get .env variables
        let page_title = std::env::var("PAGE_TITLE").unwrap_or_else(|_| "files".to_string());

        // Render template
        let template = LoginTemplate {
            page_title: &page_title
        };

        template.render().map(Html).map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                error::render_error("/", "Error rendering template"),
            )
        })
    }
}

// Form handler
pub async fn login_form(Form(form): Form<LoginForm>) -> Result<Html<String>, (StatusCode, Html<String>)> {
    if form.password == password::get_password() {
        return render_upload_template();
    }

    Err((StatusCode::UNAUTHORIZED, error::render_error("/", "Unauthorized")))
}

// Render upload template
fn render_upload_template() -> Result<Html<String>, (StatusCode, Html<String>)> {
    // Get .env variables
    let page_title = std::env::var("PAGE_TITLE").unwrap_or_else(|_| "files".to_string());
    let enable_actions = std::env::var("ENABLE_FILE_ACTIONS_DIRECTORY")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let uploads = WalkDir::new("uploads/");

    // Returns list of files
    let mut files: Vec<String> = uploads
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .filter_map(|entry| entry.file_name().to_str().map(|s| s.to_string()))
        .collect();

    files.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase())); // Sorts files alphabetically case-insensitively

    // Render template
    let template = DirectoryTemplate {
        files,
        password: password::get_password(),
        enable_actions,
        page_title: &page_title,
    };

    template.render().map(Html).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            error::render_error("/", "Error rendering template"),
        )
    })
}