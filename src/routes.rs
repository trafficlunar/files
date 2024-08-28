use std::time::Duration;

use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::{IntoResponse, Redirect}, routing::{delete, get, post}, Json, Router
};
use serde_json::json;
use tower::{buffer::BufferLayer, limit::RateLimitLayer};
use tower_http::{
    services::ServeFile,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::Level;

#[path = "password.rs"]
mod password;

mod upload;
mod delete;
mod preview;
mod raw;
mod info;

pub async fn app() -> Router {
    // let password = std::env::var("GENERATE_PASSWORD")
    //     .map_or_else(|_| "false".to_string(), |generate_password| {
    //         if generate_password == "true" {
    //             password::generate_password()
    //         } else {
    //             std::env::var("PASSWORD").expect("PASSWORD must be set when GENERATE_PASSWORD is not")
    //         }
    //     });

    Router::new()
        .route("/upload", post(upload::handler))
        .route("/delete", delete(delete::handler))
        .route_layer(ValidateRequestHeaderLayer::bearer("broken"))
        .route(
            "/",
            get(|| async { Redirect::permanent("https://github.com/axolotlmaid/files/") }),
        )
        .route_service("/favicon.ico", ServeFile::new("favicon.ico"))
        .route("/:filename", get(preview::handler))
        .route("/:filename/raw", get(raw::handler))
        .route("/:filename/info", get(info::handler))
        .fallback(handler_404)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)) // 1 GB
        // .layer(BufferLayer::new(1024))        
        // .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, Json(json!({ "success": false, "error": "404 - not found"})))
}