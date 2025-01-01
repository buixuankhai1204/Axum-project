use crate::core::app_state::AppState;
use axum::routing::{get, post, put};
use axum::Router;

pub mod handler;

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router
        .route("/api/v1/user/register_by_email", post(handler::register_by_email))
        .route("/api/v1/user/:id", get(handler::get_profile))
        .route("/api/v1/users", get(handler::get_list))
        .route("/api/v1/user/:id", put(handler::update_profile))

}
