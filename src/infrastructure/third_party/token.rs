use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::domain::authenticate::request::RefreshTokenRequest;
use crate::domain::authenticate::response::TokenResponse;
use crate::domain::entity::UserEntity;
use crate::infrastructure::persistence::redis_client;
use crate::infrastructure::persistence::repo_interface::ReadRepository;
use crate::util::claim::UserClaims;
use crate::util::constant::{
    ACCESS_TOKEN_ENCODE_KEY, EXPIRE_BEARER_TOKEN_SECS, EXPIRE_REFRESH_TOKEN_SECS,
    REFRESH_TOKEN_DECODE_KEY, REFRESH_TOKEN_ENCODE_KEY,
};
use std::sync::Arc;
use uuid::Uuid;

pub async fn service_refresh(
    state: &AppState,
    req: RefreshTokenRequest,
) -> AppResult<TokenResponse> {
    let user_claims = UserClaims::decode(&req.token, &REFRESH_TOKEN_DECODE_KEY)?.claims;
    tracing::info!("Refresh token: {user_claims:?}");
    let user_id = redis_client::session::check(&state.redis, &user_claims).await?;
    let user = UserEntity::find_data_by_uuid(&*state.db, &user_id).await;
    if user.is_none() {
        return Err(AppError::EntityNotFoundError { detail: "User not found".to_string() });
    }
    let user_clone = Arc::new(user.unwrap());
    let session_id = redis_client::session::set(&state.redis, user_clone.user_uuid).await?;
    tracing::info!("Set new session for user: {}", user_clone.user_uuid);
    let resp = service_generate_tokens(user_clone.user_uuid, user_clone.role_id, session_id)?;
    tracing::info!("Refresh token success: {user_claims:?}");
    Ok(resp)
}

pub fn service_generate_tokens(
    user_id: Uuid,
    role: i64,
    session_id: Uuid,
) -> AppResult<TokenResponse> {
    let access_token = UserClaims::new(EXPIRE_BEARER_TOKEN_SECS, user_id, session_id, role)
        .encode(&ACCESS_TOKEN_ENCODE_KEY)?;
    let refresh_token = UserClaims::new(EXPIRE_REFRESH_TOKEN_SECS, user_id, session_id, role)
        .encode(&REFRESH_TOKEN_ENCODE_KEY)?;
    Ok(TokenResponse::new(access_token, refresh_token, EXPIRE_BEARER_TOKEN_SECS.as_secs()))
}
