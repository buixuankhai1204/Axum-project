use crate::core::error::{AppError, AppResult};
use crate::infrastructure::persistence::redis_client;
use crate::infrastructure::persistence::redis_client::instance::RedisClient;
use crate::infrastructure::persistence::redis_client::services::SessionKey;
use crate::util::claim::UserClaims;
use uuid::Uuid;

pub async fn check(redis: &RedisClient, claims: &UserClaims) -> AppResult<Uuid> {
    let session_key = SessionKey { user_id: claims.uuid };
    let session_id = redis_client::services::get(redis, &session_key).await?.ok_or_else(|| {
        AppError::EntityNotAvailableError {
            detail: format!("Session not available by {}", claims.sid.to_string()),
        }
    })?;
    if claims.sid != session_id {
        tracing::info!("Session id invalid so deleting it: {session_key:?}.");
        redis_client::services::del(redis, &session_key).await?;
        return Err(AppError::InvalidSessionError("Session is Invalid".to_string()));
    }
    Ok(claims.uuid)
}

pub async fn set(redis: &RedisClient, user_id: Uuid) -> AppResult<Uuid> {
    let (key, value) = generate(user_id);
    redis_client::services::set(redis, (&key, &value)).await?;
    Ok(value)
}

pub fn generate(user_id: Uuid) -> (SessionKey, Uuid) {
    let session_id = Uuid::new_v4();
    let key = SessionKey { user_id };
    (key, session_id)
}
