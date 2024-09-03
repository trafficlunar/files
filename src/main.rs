use std::fs;

use axum::{extract::Request, ServiceExt};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;

mod metrics;
mod middleware;
mod password;
mod routes;

async fn start_main_server() {
    let app = NormalizePathLayer::trim_trailing_slash().layer(routes::app().await);

    let port = std::env::var("PORT").expect("PORT must be set.");
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + port.as_str())
        .await
        .unwrap();

    tracing::info!("listening on http://localhost:{}", port);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

async fn start_metrics_server() {
    let app = NormalizePathLayer::trim_trailing_slash().layer(metrics::app().await);

    let port = std::env::var("METRICS_PORT").expect("METRICS_PORT must be set.");
    let listener = TcpListener::bind("0.0.0.0:".to_owned() + port.as_str())
        .await
        .unwrap();

    tracing::info!("metrics listening on http://localhost:{}", port);

    axum::serve(listener, ServiceExt::<Request>::into_make_service(app))
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    let startup_text = r#"
   __ _ _
  / _(_) |
 | |_ _| | ___  ___
 |  _| | |/ _ \/ __|
 | | | | |  __/\__ \
 |_| |_|_|\___||___/

───────────────────────────────────────────────────────────
github: https://github.com/axolotlmaid/files
───────────────────────────────────────────────────────────"#;
    println!("{}", startup_text);

    dotenv().ok();

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    fs::create_dir_all("uploads/").unwrap();

    let metrics_enabled =
        std::env::var("METRICS_ENABLED").unwrap_or_else(|_| "false".to_string()) == "true";

    if metrics_enabled {
        tokio::join!(start_main_server(), start_metrics_server());
    } else {
        start_main_server().await;
    }
}
