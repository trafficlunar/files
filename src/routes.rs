use axum::{
    extract::DefaultBodyLimit,
    response::Redirect,
    routing::{delete, get, post},
    Router,
};
use tower_http::{
    services::ServeFile,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    validate_request::ValidateRequestHeaderLayer,
};
use tracing::Level;

#[path = "password.rs"]
mod password;

mod delete;
mod info;
mod notfound;
mod preview;
mod raw;
mod upload;

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
        .route("/uploads/:filename", get(preview::handler))
        .route("/uploads/:filename/raw", get(raw::handler))
        .route("/uploads/:filename/info", get(info::handler))
        .fallback(notfound::handler)
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(DefaultBodyLimit::max(1024 * 1024 * 1024)) // 1 GB
                                                          // .layer(BufferLayer::new(1024))
                                                          // .layer(RateLimitLayer::new(5, Duration::from_secs(1)))
}
