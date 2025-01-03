use crate::core::app_state::AppState;
use crate::core::error::AppResult;
use crate::core::response::{ClientResponseError, EntityResponse};
use crate::domain::user::request::AdminCreateAccountRequest;
use crate::domain::user::response::PublicProfileResponse;
use crate::domain::user::services::{service_admin_create_account, service_admin_get_list};
use crate::util::claim::UserClaims;
use crate::util::filter_and_pagination::PageQueryParam;
use axum::extract::State;
use axum::{Extension, Json};
use axum_extra::extract::Query;

#[utoipa::path(
    get,
    path = "/v1/admin/create",
    request_body = AdminCreateAccountRequest,
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success get list of users", body = [PublicProfileResponse]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn controller_admin_create_account(
    State(state): State<AppState>,
    // claims: UserClaims,
    Json(req): Json<AdminCreateAccountRequest>,
) -> AppResult<Json<PublicProfileResponse>> {
    // tracing::info!("Create new account from: {}", claims.uuid);
    // TODO: check claims is admin or not?
    match service_admin_create_account(&state, req).await {
        Ok(result) => Ok(Json(result)),
        Err(err) => {
            tracing::warn!("Unsuccessfully create new account failed: {err:?}.");
            Err(err)
        },
    }
}

#[utoipa::path(
    get,
    path = "/v1/admin/list",
    params(PageQueryParam),
    tags = ["user_service"],
    responses(
        (status = 200, description = "Success get list of users", body = [EntityResponse<Vec<PublicProfileResponse>>]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 401, description = "Unauthorized", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
    security(("jwt" = []))
)]
pub async fn controller_admin_get_list(
    State(state): State<AppState>,
    claims: UserClaims,
    Query(param): Query<PageQueryParam>,
) -> AppResult<Json<EntityResponse<Vec<PublicProfileResponse>>>> {
    tracing::info!("Get list of parameter: {:?}.", param);
    // TODO: check claims is admin or not?
    match service_admin_get_list(&state, param).await {
        Ok(results) => Ok(Json(EntityResponse {
            message: "Get all users success!".to_string(),
            total: results.iter().clone().len() as u16,
            data: Some(results),
        })),
        Err(err) => {
            tracing::warn!("Unsuccessfully get profile user: {err:?}.");
            Err(err)
        },
    }
}
