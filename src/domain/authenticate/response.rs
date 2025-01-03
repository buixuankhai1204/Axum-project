use crate::util::constant::BEARER;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[serde(tag = "type")]
pub enum LoginResponse {
    Token(TokenResponse),
    Code { message: String, expire_in: u64 },
}

impl From<TokenResponse> for LoginResponse {
    fn from(value: TokenResponse) -> Self {
        LoginResponse::Token(value)
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TokenResponse {
    pub token_type: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expire_in: u64,
}

impl TokenResponse {
    pub fn new(access_token: String, refresh_token: String, expire_in: u64) -> Self {
        Self { token_type: BEARER.to_string(), access_token, refresh_token, expire_in }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ForgetPasswordResponse {
    pub expire_in: u64,
    pub message: String,
}
