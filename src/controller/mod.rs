use crate::core::app_state::AppState;
use axum::routing::{get, post, put};
use axum::Router;

pub mod admin;
pub mod auth;
pub mod employee;
pub mod openapi;
pub mod server;
pub mod user;

pub fn build_routes() -> Router<AppState> {
    let server_routes = Router::new()
        .route("/v1/server/health_check", get(server::health_check))
        .route("/v1/server/state", get(server::server_state));

    let auth_routes = Router::new()
        .route("/v1/login_by_email", post(auth::controller_login_by_email))
        .route("/v1/refresh_token", post(auth::controller_refresh_token));

    let user_routes = Router::new()
        .route("/v1/admin/create", post(admin::user::controller_admin_create_account))
        .route("/v1/admin/list", get(admin::user::controller_admin_get_list))
        .route("/v1/me", get(user::controller_get_profile))
        .route("/v1/me", put(user::controller_update_profile))
        .route("/v1/logout", post(user::controller_logout));

    let employee_routes =
        Router::new().route("/employee", get(employee::create_new_employee_by_user_exist));

    Router::new().merge(server_routes).merge(auth_routes).merge(user_routes).merge(employee_routes)
}
