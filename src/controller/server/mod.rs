use crate::core::app_state::AppState;
use crate::core::error::AppResult;
use crate::core::response::{ClientResponseError, MessageResponse, ServiceStatusResponse};
use crate::infrastructure::persistence::redis_client::instance::RedisClientExt;
use axum::{extract::State, routing::get, Json};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[utoipa::path(
    get,
    path = "/v1/server/health_check",
    tags = ["server_service"],
    responses(
        (status = 200, description = "check service is up", body = [MessageResponse])
    )
)]
pub async fn health_check() -> AppResult<Json<MessageResponse>> {
    Ok(Json(MessageResponse::new("Ok")))
}

#[utoipa::path(
    get,
    path = "/v1/server/state",
    tags = ["server_service"],
    responses(
        (status = 200, description = "state of connection services", body = [ServiceStatusResponse]),
        (status = 500, description = "internal server error", body = [ClientResponseError])
    )
)]
pub async fn server_state(State(state): State<AppState>) -> AppResult<Json<ServiceStatusResponse>> {
    let db = state.db.ping().await;
    if let Err(err) = db.as_ref() {
        tracing::error!("Database connection failed error: {err}.");
    }
    let redis = state.redis.ping().await;
    if let Err(err) = redis.as_ref() {
        tracing::error!("Redis connection failed error: {err}.");
    }
    let resp = ServiceStatusResponse { db: db.is_ok(), redis: redis.is_ok() };
    Ok(Json(resp))
}
