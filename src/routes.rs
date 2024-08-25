use axum::{
    response::Redirect,
    routing::{delete, get, post},
    Router,
};
use tower_http::{
    services::ServeFile,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
};
use tracing::Level;

mod delete;
mod preview;
mod raw;
mod upload;

pub async fn app() -> Router {
    Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent("https://github.com/axolotlmaid/files/") }),
        )
        .route_service("/favicon.ico", ServeFile::new("favicon.ico"))
        .route("/upload", post(upload::handler))
        .route("/delete", delete(delete::handler))
        .route("/:filename", get(preview::handler))
        .route("/:filename/raw", get(raw::handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
}
