use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use serde_json::{json, Value};

use crate::password;

// Middleware for handling authorization
pub async fn middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    // Get Authorization header
    if let Some(authorization) = request.headers().get("Authorization") {
        // Convert header value to str
        if let Ok(auth_value) = authorization.to_str() {
            // Check password is correct
            if auth_value == password::get_password() {
                // Run request normally
                return Ok(next.run(request).await);
            }
        }
    }

    // Return error if anything fails
    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({ "success": false, "error": "Unauthorized" })),
    ))
}
