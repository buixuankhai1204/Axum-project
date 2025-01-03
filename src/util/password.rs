use super::hash;
use crate::core::error::{AppError, AppResult};

pub async fn hash(password: String) -> AppResult<String> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    let password = jh.await??;
    Ok(password)
}

pub async fn verify(password: String, hashed_pass: String) -> AppResult {
    let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hashed_pass));
    if let Err(err) = jh.await? {
        tracing::debug!("The password is not correct: {err}");
        Err(AppError::BadRequestError("The password is not correct!".to_string()))
    } else {
        Ok(())
    }
}
