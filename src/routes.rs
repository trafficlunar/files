use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    http::StatusCode,
    middleware,
    response::Redirect,
    routing::{delete, get, post},
    Router,
};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{
    services::ServeFile,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::Level;

mod api;
mod files;
mod notfound;
#[path = "password.rs"]
mod password;
use crate::metrics;

pub async fn app() -> Router {
    let password = password::get_password();

    Router::new()
        .route("/upload", post(api::upload::handler))
        .route("/delete", delete(api::delete::handler))
        .route_layer(ValidateRequestHeaderLayer::bearer(&password))
        .route(
            "/",
            get(|| async { Redirect::permanent("https://github.com/axolotlmaid/files/") }),
        )
        .route_service("/favicon.ico", ServeFile::new("favicon.ico"))
        .route("/uploads", get(files::directory::handler))
        .route("/uploads/:filename", get(files::preview::handler))
        .route("/uploads/:filename/raw", get(files::raw::handler))
        .route("/uploads/:filename/info", get(files::info::handler))
        .fallback(notfound::handler)
        .route_layer(middleware::from_fn(metrics::track_metrics))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(HandleErrorLayer::new(|err| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(DefaultBodyLimit::max(1000 * 1000 * 1000)) // 1 GB
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(5)))
                .into_inner(),
        )
}
