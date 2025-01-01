use crate::core::app_state::AppState;
use crate::core::error::{AppError, AppResult};
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
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
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // user id
    pub uid: i64,
    // session id
    pub sid: Uuid,
    // role user
    pub rol: String,
}

impl UserClaims {
    pub fn new(duration: Duration, user_id: i64, session_id: Uuid, role: String) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uid: user_id,
            sid: session_id,
            rol: role,
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

#[async_trait::async_trait]
impl FromRequestParts<AppState> for UserClaims {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // let TypedHeader(Authorization(bearer)) =
        //     parts.extract::<TypedHeader<Authorization<Bearer>>>().await?;
        // let user_claims = UserClaims::decode(bearer.token(), &ACCESS_TOKEN_DECODE_KEY)?.claims;
        // service::session::check(&state.redis, &user_claims).await?;
        // Ok(user_claims)
        todo!()
    }
}

pub trait UserClaimsRequest {
    fn get_user_id(&self) -> AppResult<i64>;
    fn get_user_claims(&self) -> AppResult<UserClaims>;
}

impl UserClaimsRequest for axum::extract::Request {
    fn get_user_id(&self) -> AppResult<i64> {
        self.extensions()
            .get::<UserClaims>()
            .map(|u| u.uid)
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }

    fn get_user_claims(&self) -> AppResult<UserClaims> {
        self.extensions()
            .get::<UserClaims>()
            .cloned()
            .ok_or_else(|| AppError::UnauthorizedError("User Must Login".to_string()))
    }
}
