use axum::{extract::Request, ServiceExt};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = NormalizePathLayer::trim_trailing_slash().layer(routes::app().await);

    let port = std::env::var("PORT").expect("PORT must be set.");
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + port.as_str())
        .await
        .unwrap();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    tracing::info!("listening on http://localhost:{}", port);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}
