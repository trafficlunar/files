use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;

// Fallback route handler

// Adding this to `/middleware/status_code.rs` will result in other code sending the status code NOT_FOUND
// to have their response changed. In other words, `preview.rs` tries to send a HTML 404 page with the status
// code NOT_FOUND and if I add this handler to `status_code.rs` that HTML response will be intercepted and
// the user will be sent a JSON error instead.
pub async fn handler() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "success": false, "error": "Route not found"})),
    )
}