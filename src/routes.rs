use axum::{routing::get, Router};

pub async fn app() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> &'static str {
    "Hello, World!"
}