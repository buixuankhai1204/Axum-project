use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use crate::domain::authenticate::request::LoginByEmailRequest;
use crate::domain::authenticate::response::TokenResponse;
use crate::domain::entity::UserEntity;
use crate::infrastructure::persistence::redis_client;
use crate::infrastructure::persistence::redis_client::services::SessionKey;
use crate::infrastructure::third_party;
use crate::util::password;
use std::sync::Arc;
use uuid::Uuid;

pub async fn service_login_by_email(
    state: &AppState,
    req: LoginByEmailRequest,
) -> AppResult<TokenResponse> {
    tracing::info!("User login request :{req:?}.");
    let user = UserEntity::repo_find_by_email_and_status(&*state.db, &req.email, &1).await;
    if user.is_none() {
        return Err(AppError::EntityNotFoundError { detail: "User not found".to_string() });
    }
    let user_unwrap = user.unwrap();
    password::verify(req.password, user_unwrap.password).await?;
    let session_id = redis_client::session::set(&state.redis, user_unwrap.user_uuid).await?;
    let res = third_party::token::service_generate_tokens(
        user_unwrap.user_uuid,
        user_unwrap.role_id,
        session_id,
    )?;
    Ok(res)
}

pub async fn service_logout(state: &AppState, user_id: Uuid) -> AppResult<()> {
    tracing::info!("Logout user id: {user_id}");
    let key = SessionKey { user_id };
    redis_client::services::del(&state.redis, &key).await?;
    Ok(())
}
