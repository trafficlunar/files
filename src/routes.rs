use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{delete, get, post, put},
    Router,
};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};
use tower_http::{
    services::ServeFile,
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod api;
mod files;
mod notfound;

use crate::middleware;
use crate::{metrics, password};

pub async fn app() -> Router {
    let base_api_router = Router::new()
        .route("/upload", post(api::upload::handler))
        .route("/delete", delete(api::delete::handler))
        .route("/rename", put(api::rename::handler));

    // Get .env variables
    let protect_directory = std::env::var("PROTECT_DIRECTORY")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    // Put the route_layer above or below `/uploads` depending on `protect_directory` and `password` value
    let api_router = if password::get_password() == "" {
        base_api_router.route("/uploads", get(api::directory::handler))
    } else if protect_directory {
        base_api_router
            .route("/uploads", get(api::directory::handler))
            .route_layer(axum::middleware::from_fn(
                middleware::authorization::middleware,
            ))
    } else {
        base_api_router
            .route_layer(axum::middleware::from_fn(
                middleware::authorization::middleware,
            ))
            .route("/uploads", get(api::directory::handler))
    };

    Router::new()
        .route(
            "/",
            get(files::directory::handler).post(files::directory::login_form),
        )
        .route("/:filename", get(files::preview::handler))
        .route("/:filename/raw", get(files::raw::handler))
        .route("/:filename/info", get(files::info::handler))
        .route_service("/favicon.ico", ServeFile::new("favicon.ico"))
        .route_service("/style.css", ServeFile::new("style.css"))
        .nest("/api", api_router)
        .fallback(notfound::handler)
        .route_layer(axum::middleware::from_fn(metrics::track_metrics))
        .layer(
            ServiceBuilder::new()
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                        .on_response(DefaultOnResponse::new().level(Level::INFO)),
                )
                .layer(axum::middleware::from_fn(
                    middleware::status_code::middleware,
                ))
                .layer(HandleErrorLayer::new(|err| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Unhandled error: {}", err),
                    )
                }))
                .layer(DefaultBodyLimit::max(1000 * 1000 * 1000)) // Allow files to be 1 GB in space
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(5, Duration::from_secs(5)))
                .layer(TimeoutLayer::new(Duration::from_secs(30)))
                .into_inner(),
        )
}
