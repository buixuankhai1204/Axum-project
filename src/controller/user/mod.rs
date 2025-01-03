use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::core::response::{ClientResponseError, EntityResponse, MessageResponse};
use crate::domain::authenticate::services::service_logout;
use crate::domain::user::request::UpdateProfileRequest;
use crate::domain::user::response::PublicProfileResponse;
use crate::domain::user::services::{
    service_admin_get_list, service_get_profile, service_update_profile,
};
use crate::util::claim::UserClaims;
use crate::util::filter_and_pagination::PageQueryParam;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use axum_extra::extract::Query;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/v1/me",
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success get user profile", body = [PublicProfileResponse]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn controller_get_profile(
    State(state): State<AppState>,
    claims: UserClaims,
) -> AppResult<Json<PublicProfileResponse>> {
    tracing::info!("Get profile user id: {}.", claims.uuid);
    match service_get_profile(&state, claims.uuid).await {
        Ok(resp) => {
            tracing::info!("Success get profile user: {}", claims.uuid);
            Ok(Json(resp))
        },
        Err(err) => {
            tracing::warn!("Unsuccessfully get profile user: {err:?}.");
            Err(err)
        },
    }
}

#[utoipa::path(
    put,
    path = "/v1/me",
    request_body = UpdateProfileRequest,
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success update profile information", body = MessageResponse),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn controller_update_profile(
    State(state): State<AppState>,
    claims: UserClaims,
    Json(req): Json<UpdateProfileRequest>,
) -> AppResult<Json<MessageResponse>> {
    tracing::info!("Update profile user_id: {}.", claims.uuid);
    if req.validate().is_err() {
        return Err(AppError::BadRequestError(req.validate().unwrap_err().to_string()));
    }
    match service_update_profile(&state, claims.uuid, req).await {
        Ok(_) => {
            tracing::info!("Success update profile user id: {}.", claims.uuid);
            Ok(Json(MessageResponse::new("User profile updated.")))
        },
        Err(err) => {
            tracing::info!("Unsuccessful update profile user: {err:?}");
            Err(err)
        },
    }
}

#[utoipa::path(
    post,
    path = "/v1/logout",
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success logout", body = [MessageResponse]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn controller_logout(
    State(state): State<AppState>,
    claims: UserClaims,
) -> AppResult<Json<MessageResponse>> {
    tracing::info!("Logout user id: {}", claims.uuid);
    match service_logout(&state, claims.uuid).await {
        Ok(_) => {
            tracing::info!("Success logout user id: {}", claims.uuid);
            Ok(Json(MessageResponse::new("This user has successfully logged out!")))
        },
        Err(err) => {
            tracing::error!("Unsuccessfully logout user error: {err:?}");
            Err(err)
        },
    }
}
