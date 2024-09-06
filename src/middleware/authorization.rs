use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use serde_json::{json, Value};

use crate::password;

pub async fn middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    if let Some(authorization) = request.headers().get("Authorization") {
        if let Ok(auth_value) = authorization.to_str() {
            if auth_value == password::get_password() {
                return Ok(next.run(request).await);
            }
        }
    }

    Err((
        StatusCode::UNAUTHORIZED,
        Json(json!({ "success": false, "error": "Unauthorized" })),
    ))
}
