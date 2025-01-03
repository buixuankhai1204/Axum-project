use crate::core::app_state::AppState;
use crate::core::error::AppResult;
use crate::core::response::{ClientResponseError, EntityResponse};
use crate::domain::employee::request::{
    CreateNewEmployeeByUserUuidRequest, CreateNewEmployeeRequest, DeleteEmployeeRequest,
    UpdateEmployeeRequest,
};
use crate::domain::employee::services::{
    service_create_new_employee, service_create_new_employee_by_user_exist,
    service_delete_employee, service_update_employee,
};
use axum::extract::State;
use axum::Json;
use tower_http::validate_request::ValidateRequest;
use tracing::{info, warn};

#[utoipa::path(
    post,
    tags = ["employee_service"],
    request_body = CreateNewEmployeeByUserUuidRequest,
    path = "/v1/employee/create_by_exist_user",
    responses(
        (status = 200, description = "Success create employee from exist user", body = [EntityResponse<i64>]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn create_new_employee_by_user_exist(
    State(state): State<AppState>,
    Json(req): Json<CreateNewEmployeeByUserUuidRequest>,
) -> AppResult<Json<EntityResponse<i64>>> {
    info!("Register new employee with request: {req:?}");
    return match service_create_new_employee_by_user_exist(&state, &req).await {
        Ok(value) => Ok(Json(EntityResponse {
            message: "create new employee from existed 123123!".to_string(),
            data: Some(value.clone()),
            total: 1,
        })),
        Err(e) => {
            warn!("Unsuccessfully get profile 123123: {e:?}.");
            Err(e)
        },
    };
}

#[utoipa::path(
    post,
    tags = ["employee_service"],
    request_body = CreateNewEmployeeRequest,
    path = "/v1/employee/new",
    responses(
        (status = 200, description = "Success create new employee", body = [EntityResponse<i64>]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn create_new_employee(
    State(state): State<AppState>,
    Json(req): Json<CreateNewEmployeeRequest>,
) -> AppResult<Json<EntityResponse<i64>>> {
    return match service_create_new_employee(&state, &req).await {
        Ok(value) => Ok(Json(EntityResponse {
            message: "create new employee from existed 123123!".to_string(),
            data: Some(value),
            total: 1,
        })),
        Err(e) => {
            warn!("Unsuccessfully get profile 123123: {e:?}.");
            Err(e)
        },
    };
}

#[utoipa::path(
    put,
    tags = ["employee_service"],
    request_body = UpdateEmployeeRequest,
    path = "/v1/employee/update",
    responses(
        (status = 200, description = "Success create new employee", body = [EntityResponse<i64>]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn update_employee(
    State(state): State<AppState>,
    Json(req): Json<UpdateEmployeeRequest>,
) -> AppResult<Json<EntityResponse<i64>>> {
    info!("Register new employee with request: {req:?}");
    return match service_update_employee(&state, &req).await {
        Ok(value) => Ok(Json(EntityResponse {
            message: "create new employee from existed 123123!".to_string(),
            data: Some(value),
            total: 1,
        })),
        Err(e) => {
            warn!("Unsuccessfully get profile 123123: {e:?}.");
            Err(e)
        },
    };
}

#[utoipa::path(
    put,
    tags = ["employee_service"],
    request_body = DeleteEmployeeRequest,
    path = "/v1/employee/delete",
    responses(
        (status = 200, description = "Success create new employee", body = [EntityResponse<i64>]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn delete_employee(
    State(state): State<AppState>,
    Json(req): Json<DeleteEmployeeRequest>,
) -> AppResult<Json<EntityResponse<i64>>> {
    info!("Register new employee with request: {req:?}");
    match service_delete_employee(&state, &req).await {
        Ok(value) => Ok(Json(EntityResponse {
            message: "create new employee from existed 123123!".to_string(),
            data: Some(value),
            total: 1,
        })),
        Err(e) => {
            warn!("Unsuccessfully get profile 123123: {e:?}.");
            Err(e)
        },
    }
}
