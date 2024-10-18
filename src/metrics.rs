use std::{future::ready, path::PathBuf, time::Instant};

use axum::{
    extract::{MatchedPath, Request},
    middleware::Next,
    response::IntoResponse,
    routing::get,
    Router,
};
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use walkdir::WalkDir;

// Metrics router
pub async fn app() -> Router {
    let recorder_handle = setup_metrics_recorder();

    Router::new()
        .route("/", get(handler_root))
        .route("/metrics", get(move || ready(recorder_handle.render())))
}

async fn handler_root() -> impl IntoResponse {
    "https://github.com/trafficlunar/files - metrics server"
}

// Metrics handlers
fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

pub async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start: Instant = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    let uploads_path = PathBuf::from("uploads");
    let uploads = WalkDir::new(&uploads_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.path() != uploads_path)
        .count();

    metrics::gauge!("uploads").set(uploads as f64);
    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    response
}
