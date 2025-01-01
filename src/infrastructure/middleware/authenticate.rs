use crate::core::error::AppResult;
use axum::{extract::Request, middleware::Next, response::Response};
use log::warn;

pub async fn mw_authenticate(req: Request, next: Next) -> AppResult<Response> {
    tracing::debug!("->> MIDDLEWARE AUTH");
    if req.uri().path().to_string().contains("/swagger-ui") {
        return Ok(next.run(req).await);
    }
    Ok(next.run(req).await)
}
