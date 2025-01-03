use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::core::response::{ClientResponseError, EntityResponse, MessageResponse};
use crate::domain::authenticate::request::{LoginByEmailRequest, RefreshTokenRequest};
use crate::domain::authenticate::response::{LoginResponse, TokenResponse};
use crate::domain::authenticate::services::{service_login_by_email, service_logout};
use crate::infrastructure::third_party::token::service_refresh;
use crate::util::claim::UserClaims;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use validator::Validate;

#[utoipa::path(
    post,
    path = "/v1/login_by_email",
    request_body = LoginByEmailRequest,
    tags = ["auth_service"],
    responses(
        (status = 200, description = "Success login", body = [LoginResponse]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 404, description = "Account not found", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    )
)]
pub async fn controller_login_by_email(
    State(state): State<AppState>,
    Json(req): Json<LoginByEmailRequest>,
) -> AppResult<Json<LoginResponse>> {
    tracing::info!("Login by email with request: {req:?}.");
    if req.validate().is_err() {
        return Err(AppError::BadRequestError(req.validate().unwrap_err().to_string()));
    }
    match service_login_by_email(&state, req).await {
        Ok(res) => {
            tracing::info!("Success login!");
            Ok(Json(LoginResponse::Token(res)))
        },
        Err(err) => {
            tracing::error!("Unsuccessfully login by email error: {err:?}");
            Err(err)
        },
    }
}

#[utoipa::path(
    post,
    path = "/v1/refresh_token",
    request_body = RefreshTokenRequest,
    tags = ["auth_service"],
    responses(
        (status = 200, description = "Success get new access token and refresh token", body = [TokenResponse]),
        (status = 400, description = "Invalid data input", body = [ClientResponseError]),
        (status = 404, description = "Account not found", body = [ClientResponseError]),
        (status = 500, description = "Internal server error", body = [ClientResponseError])
    ),
)]
pub async fn controller_refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> AppResult<Json<TokenResponse>> {
    tracing::info!("Refresh token with request: {req:?}.");
    if req.validate().is_err() {
        return Err(AppError::BadRequestError(req.validate().unwrap_err().to_string()));
    }
    match service_refresh(&state, req).await {
        Ok(res) => {
            tracing::info!("Success refresh token");
            Ok(Json(res))
        },
        Err(err) => {
            tracing::error!("Unsuccessfully refresh token error: {err:?}");
            Err(err)
        },
    }
}
