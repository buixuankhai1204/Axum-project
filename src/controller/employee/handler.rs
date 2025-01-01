use crate::core::app_state::AppState;
use crate::core::error::AppResult;
use crate::core::response::{ClientResponseError, EntityResponse};
use crate::domain::employee::request::CreateNewEmployeeByUserUuidRequest;
use crate::domain::employee::services::service_create_new_employee_by_user_exist;
use crate::domain::user::request::RegisterRequest;
use crate::domain::user::response::RegisterResponse;
use crate::domain::user::services::service_register_by_email;
use axum::extract::State;
use axum::Json;
use tower_http::validate_request::ValidateRequest;
use tracing::{info, warn};

#[utoipa::path(
    post,
    request_body = CreateNewEmployeeByUserUuidRequest,
    path = "/api/v1/employee/create_by_exist_user",
    responses(
        (status = 200, description = "Success register user", body = [RegisterResponse]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn create_new_employee_by_user_exist(
    State(state): State<AppState>,
    Json(req): Json<CreateNewEmployeeByUserUuidRequest>,
) -> AppResult<Json<EntityResponse<i64>>> {
    info!("Register new user with request: {req:?}");
    return match service_create_new_employee_by_user_exist(&state, &req).await {
        Ok(value) => Ok(Json(EntityResponse {
            message: "create new employee from existed user!".to_string(),
            data: Some(value.clone()),
            total: 1,
        })),
        Err(e) => {
            warn!("Unsuccessfully get profile user: {e:?}.");
            Err(e)
        },
    };
}