use axum::{
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub async fn handler_404(uri: Uri, req_method: Method) -> Response {
    let body = Json(json!({
      "message" : "Route Not Found",
      "uri" : uri.to_string(),
      "method" : req_method.to_string()
    }));
    (StatusCode::NOT_FOUND, body).into_response()
}
