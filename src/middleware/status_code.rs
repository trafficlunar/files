use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response, Json};
use serde_json::{json, Value};

// This middleware intercepts every request and changes the response depending on that request's status code
// There is probably a better way to do this but I couldn't find a documented solution therefore I created this
pub async fn middleware(
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<Value>)> {
    let response = next.run(request).await;

    match response.status() {
        StatusCode::UNAUTHORIZED => Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({ "success": false, "error": "Unauthorized" })),
        )),
        StatusCode::METHOD_NOT_ALLOWED => Err((
            StatusCode::METHOD_NOT_ALLOWED,
            Json(json!({ "success": false, "error": "Wrong method used for route" })),
        )),
        _ => Ok(response),
    }
}
