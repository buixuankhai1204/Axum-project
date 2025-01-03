use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate)]
#[serde(tag = "type")]
pub struct LoginByEmailRequest {
    #[validate(email)]
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, IntoParams)]
pub struct RefreshTokenRequest {
    #[validate(length(min = 30))]
    pub token: String,
}

#[derive(Debug, Deserialize, ToSchema, Validate, IntoParams)]
pub struct ForgetPasswordQueryParam {
    #[validate(email)]
    pub email: String,
}
