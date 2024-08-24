use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use tower_http::{
    services::ServeFile,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod upload;
mod preview;
mod raw;

pub async fn app() -> Router {
    Router::new()
        .route("/", get(root))
        .route_service("/favicon.ico", ServeFile::new("favicon.ico"))
        .route("/upload", get(upload::route))
        .route("/:filename", get(preview::route))
        .route("/:filename/raw", get(raw::route))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}

async fn root() -> Json<Value> {
    Json(json!({ "message": "hello world." }))
}
