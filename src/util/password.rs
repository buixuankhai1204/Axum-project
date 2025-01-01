use super::hash;
use crate::core::error::{invalid_input_error, AppResult};

pub async fn hash(password: String) -> AppResult<String> {
    let jh = tokio::task::spawn_blocking(move || hash::argon_hash(password));
    let password = jh.await??;
    Ok(password)
}

pub async fn verify(password: String, hashed_pass: String) -> AppResult {
    let jh = tokio::task::spawn_blocking(move || hash::argon_verify(password, hashed_pass));
    if let Err(e) = jh.await? {
        tracing::debug!("The password is not correct: {e}");
        Err(invalid_input_error("password", "The password is not correct."))
    } else {
        Ok(())
    }
}
