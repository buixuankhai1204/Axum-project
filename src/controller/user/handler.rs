use crate::core::app_state::AppState;
use crate::core::error::AppResult;
use crate::core::response::{ClientResponseError, EntityResponse, MessageResponse};
use crate::domain::user::request::{RegisterRequest, UpdateProfileRequest};
use crate::domain::user::response::{
    GetUserListResponse, GetUserResponse, ProfileResponse, RegisterResponse,
};
use crate::domain::user::services::{
    service_get_list, service_get_profile, service_register_by_email, service_update_profile,
};
use crate::util::claim::UserClaims;
use crate::util::filter_and_pagination::PageQueryParam;
use axum::extract::{Path, State};
use axum::Json;
use axum_extra::extract::Query;
use garde::Validate;
use tracing::{info, warn};

#[utoipa::path(
    post,
    request_body = RegisterRequest,
    path = "/api/v1/user/register_by_email",
    responses(
        (status = 200, description = "Success register user", body = [RegisterResponse]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn register_by_email(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> AppResult<Json<RegisterResponse>> {
    info!("Register new user with request: {req:?}");
    req.validate()?;
    match service_register_by_email(state, req).await {
        Ok(user) => {
            info!("Successfully register user: {}", user.username);
            let resp = RegisterResponse { username: user.username };
            Ok(Json(resp))
        },
        Err(err) => {
            warn!("Unsuccessfully register user: {err:?}");
            Err(err)
        },
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/user/:id",
    responses(
        (status = 200, description = "Success get user profile", body = [ProfileResponse]),
        (status = 401, description = "Unauthorized user", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_profile(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> AppResult<Json<ProfileResponse>> {
    info!("Get profile user id: {}.", id);
    match service_get_profile(&state, id).await {
        Ok(resp) => {
            info!("Success get profile user: {}", id);
            Ok(Json(resp))
        },
        Err(e) => {
            warn!("Unsuccessfully get profile user: {e:?}.");
            Err(e)
        },
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/users",
    params(PageQueryParam),
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success get list of users", body = [GetUserListResponse]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 401, description = "Unauthorized user", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn get_list(
    State(state): State<AppState>,
    Query(param): Query<PageQueryParam>,
) -> AppResult<Json<EntityResponse<Vec<GetUserResponse>>>> {
    info!("Get list of parameter: {:?}.", param);
    match service_get_list(&state, param).await {
        Ok(value) => {
            return Ok(Json(EntityResponse {
                message: "get all users success!".to_string(),
                data: Some(value.clone()),
                total: value.len() as u16,
            }))
        },
        Err(e) => {
            warn!("Unsuccessfully get profile user: {e:?}.");
            Err(e)
        },
    }
}

#[utoipa::path(
    put,
    path = "/api/v1/user/:id",
    request_body = UpdateProfileRequest,

    responses(
        (status = 200, description = "Success update profile information", body = MessageResponse),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 401, description = "Unauthorized user", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn update_profile(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<MessageResponse>> {
    info!("Update profile user_id: {}.", id);
    match service_update_profile(&state, id, req).await {
        Ok(_) => {
            info!("Success update profile user user_id: {}.", id);
            Ok(Json(MessageResponse::new("User profile updated.")))
        },
        Err(e) => {
            info!("Unsuccessful update profile user: {e:?}");
            Err(e)
        },
    }
}
