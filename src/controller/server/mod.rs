use crate::core::app_state::AppState;
use axum::routing::get;
use axum::Router;

pub mod handler;

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/v1/server/health_check", get(handler::health_check))
        .route("/api/v1/server/state", get(handler::server_state))
}
