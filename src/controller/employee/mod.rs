use crate::core::app_state::AppState;
use axum::routing::get;
use axum::Router;

pub mod handler;

pub fn add_routers(router: Router<AppState>) -> Router<AppState> {
    router.route("/employee", get(handler::create_new_employee_by_user_exist))
}
