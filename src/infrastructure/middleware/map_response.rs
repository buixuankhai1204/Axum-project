use crate::core::error::{AppError, AppResult};
use axum::{
    body::to_bytes,
    extract::State,
    http::{Method, StatusCode, Uri},
    response::{IntoResponse, Response},
    Json,
};
use log::warn;
use serde::Serialize;
use serde_json::{json, to_value, Value};
use std::sync::Arc;
use tracing::{debug, error};
use uuid::Uuid;

pub async fn mw_map_response(uri: Uri, req_method: Method, res: Response) -> Response {
    if uri.path().to_string().contains("/swagger-ui") {
        return res;
    }

    let uuid = Uuid::new_v4();
    let web_error = res.extensions().get::<Arc<AppError>>().map(Arc::as_ref);
    let client_status_error = web_error.map(|err| err.status_and_error());

    match client_status_error {
        Some((status_code, client_error)) => {
            let client_error = to_value(client_error).ok();
            let message = client_error.as_ref().and_then(|v| v.get("message"));
            let details = client_error.as_ref().and_then(|v| v.get("details"));

            let error_body = json!({
                "error" : {
                    "detail" : details,
                    "message" : message,
                },
                "result": null,
                "metadata" : null
            });
            let _ = log_request(uuid, uri, req_method, error_body.clone(), 0).await;
            (status_code, Json(error_body)).into_response()
        },
        None => {
            let status = res.status();
            let body = to_bytes(res.into_body(), usize::MAX).await.unwrap_or_default();
            let body_string = String::from_utf8(body.to_vec()).unwrap_or_default();
            let data: Value = serde_json::from_str(&body_string).unwrap_or(Value::Null);
            let json_response: Value = match data.clone() {
                Value::Object(map) => {
                    if map.contains_key("data") && map.contains_key("metadata") {
                        json!({
                            "error": null,
                            "result" : map.get("data").cloned().unwrap_or(Value::Null),
                            "metadata" : map.get("metadata").cloned().unwrap_or(Value::Null) // pagination
                        })
                    } else {
                        json!({
                            "error": null,
                            "result" : data,
                            "metadata" : null
                        })
                    }
                },
                _ => {
                    json!({
                        "error": null,
                        "result" : data,
                        "metadata" : null
                    })
                },
            };
            let _ = log_request(uuid, uri, req_method, json_response.clone(), 1).await;
            (status, Json(json_response)).into_response()
        },
    }
}

async fn log_request(
    uuid: Uuid,
    uri: Uri,
    req_method: Method,
    error_data: Value,
    status: u8,
) -> AppResult<()> {
    let log = RequestLogLine {
        uuid: uuid.to_string(),
        http_method: req_method.to_string(),
        http_path: uri.to_string(),
        error_data,
        status,
    };

    if status == 0 {
        error!("\n{}", json!(log));
    } else {
        debug!("\n{}", json!(log));
    }
    Ok(())
}

#[derive(Serialize)]
struct RequestLogLine {
    uuid: String, // uuid string formatted
    // -- http request attributes.
    http_path: String,
    http_method: String,
    // -- Errors attributes.
    error_data: Value,
    status: u8,
}

pub async fn handler_404(uri: Uri, req_method: Method) -> Response {
    let body = Json(json!({
      "message" : "Route Not Found",
      "uri" : uri.to_string(),
      "method" : req_method.to_string()
    }));
    (StatusCode::NOT_FOUND, body).into_response()
}
