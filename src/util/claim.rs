use crate::core::error::{AppError, AppResult};
use axum::RequestPartsExt;
use chrono::Utc;
use jsonwebtoken::Header;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, TokenData, Validation};
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde::Serialize;
use std::time::Duration;
use utoipa::ToSchema;
use uuid::Uuid;

pub static DECODE_HEADER: Lazy<Validation> = Lazy::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: Lazy<Header> = Lazy::new(|| Header::new(Algorithm::RS256));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, ToSchema)]
pub struct UserClaims {
    pub iat: i64,
    pub exp: i64,
    pub uuid: Uuid,
    pub sid: Uuid,
    pub role: i64,
}

impl UserClaims {
    pub fn new(duration: Duration, user_uid: Uuid, session_id: Uuid, role: i64) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uuid: user_uid,
            sid: session_id,
            role,
        }
    }

    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<UserClaims>(token, key, &DECODE_HEADER)
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&ENCODE_HEADER, self, key)
    }
}

pub trait UserClaimsRequest {
    fn get_user_id(&self) -> AppResult<Uuid>;
    fn get_user_claims(&self) -> AppResult<UserClaims>;
}

impl UserClaimsRequest for axum::extract::Request {
    fn get_user_id(&self) -> AppResult<Uuid> {
        self.extensions()
            .get::<UserClaims>()
            .map(|u| u.uuid)
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }

    fn get_user_claims(&self) -> AppResult<UserClaims> {
        self.extensions()
            .get::<UserClaims>()
            .cloned()
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }
}
