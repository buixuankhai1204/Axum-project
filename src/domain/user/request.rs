use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, Validate, utoipa::ToSchema)]
pub struct RegisterRequest {
    #[garde(ascii, length(min = 3, max = 20))]
    pub username: String,
    #[garde(email)]
    pub email: String,
    #[garde(length(min = 8))]
    pub password: String,
}

impl RegisterRequest {
    pub fn new(username: &str, email: &str, password: &str) -> Self {
        Self {
            password: password.to_string(),
            username: username.to_string(),
            email: email.to_string(),
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Validate, Default)]
pub struct UpdateProfileRequest {
    #[garde(skip)]
    pub username: Option<String>,
    #[garde(length(min = 8))]
    pub password: Option<String>,
    #[garde(skip)]
    pub is_two_fa: Option<bool>,
    #[garde(skip)]
    pub is_private: Option<bool>,
}
