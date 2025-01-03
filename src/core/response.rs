use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MessageResponse {
    pub message: String,
}

impl MessageResponse {
    pub fn new<S: Into<String>>(message: S) -> Self {
        Self { message: message.into() }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ServiceStatusResponse {
    pub db: bool,
    pub redis: bool,
}

#[derive(Deserialize, Serialize, Clone, Debug, ToSchema)]
pub struct EntityResponse<T> {
    pub message: String,
    pub data: Option<T>,
    pub total: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, ToSchema)]
#[serde(tag = "code_message", content = "message")]
pub enum ClientResponseError {
    EntityNotFound { detail: String },
    EntityNotAvailable { detail: String },
    EntityAlreadyExists { detail: String },
    BadRequest { detail: String },
    Unauthorized,
    AccountForbidden,
    PermissionDenied,
    InternalServerError,
}
