use crate::core::response::ClientResponseError;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub type AppResult<T = ()> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Entity not found!")]
    EntityNotFoundError { detail: String },
    #[error("Entity not available!")]
    EntityNotAvailableError { detail: String },
    #[error("Entity already exists!")]
    EntityExistsError { detail: String },
    #[error("{0}")]
    PermissionDeniedError(String),
    #[error("{0}")]
    UserNotActiveError(String),
    #[error("{0}")]
    InvalidSessionError(String),
    #[error("{0}")]
    ConflictError(String),
    #[error("{0}")]
    UnauthorizedError(String),
    #[error("Bad request {0}")]
    BadRequestError(String),
    #[error("{0}")]
    InvalidPayloadError(String),
    #[error("{0}")]
    HashError(String),
    #[error(transparent)]
    DatabaseError(#[from] sea_orm::error::DbErr),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    JwtError(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    HttpClientError(#[from] reqwest::Error),
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
    #[error(transparent)]
    ConfigError(#[from] config::ConfigError),
    #[error(transparent)]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error(transparent)]
    LetterError(#[from] lettre::error::Error),
    #[error(transparent)]
    ParseJsonError(#[from] serde_json::Error),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),
    #[error(transparent)]
    SpawnTaskError(#[from] tokio::task::JoinError),
    #[error(transparent)]
    TeraError(#[from] tera::Error),
    #[error(transparent)]
    Base64Error(#[from] base64::DecodeError),
    #[error(transparent)]
    StrumParseError(#[from] strum::ParseError),
    #[error(transparent)]
    SystemTimeError(#[from] std::time::SystemTimeError),
    #[error(transparent)]
    AxumError(#[from] axum::Error),
    #[error(transparent)]
    UnknownError(#[from] anyhow::Error),
    #[error(transparent)]
    Infallible(#[from] std::convert::Infallible),
    #[error(transparent)]
    TypeHeaderError(#[from] axum_extra::typed_header::TypedHeaderRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, body) = self.status_and_error();
        (status_code, Json(body)).into_response()
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(value: argon2::password_hash::Error) -> Self {
        AppError::HashError(value.to_string())
    }
}

impl AppError {
    pub fn status_and_error(&self) -> (StatusCode, ClientResponseError) {
        use AppError::*;
        match self {
            InvalidPayloadError(err) => (
                StatusCode::BAD_REQUEST,
                ClientResponseError::BadRequest { detail: err.to_string() },
            ),
            BadRequestError(err) => (
                StatusCode::BAD_REQUEST,
                ClientResponseError::BadRequest { detail: err.to_string() },
            ),
            EntityNotFoundError { detail } => (
                StatusCode::BAD_REQUEST,
                ClientResponseError::EntityNotFound { detail: detail.to_string() },
            ),
            EntityNotAvailableError { detail } => (
                StatusCode::BAD_REQUEST,
                ClientResponseError::EntityNotAvailable { detail: detail.to_string() },
            ),
            EntityExistsError { detail } => (
                StatusCode::CONFLICT,
                ClientResponseError::EntityAlreadyExists { detail: detail.to_string() },
            ),
            AxumError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            ConfigError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            AddrParseError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            ParseJsonError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            StrumParseError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            HttpClientError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            SystemTimeError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            SpawnTaskError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            PermissionDeniedError(_err) => {
                (StatusCode::FORBIDDEN, ClientResponseError::PermissionDenied)
            },
            InvalidSessionError(err) => (
                StatusCode::BAD_REQUEST,
                ClientResponseError::BadRequest { detail: err.to_string() },
            ),
            ConflictError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            UserNotActiveError(_err) => {
                (StatusCode::FORBIDDEN, ClientResponseError::AccountForbidden)
            },
            UnauthorizedError(_err) => {
                (StatusCode::UNAUTHORIZED, ClientResponseError::Unauthorized)
            },
            UuidError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            JwtError(_err) => (StatusCode::UNAUTHORIZED, ClientResponseError::Unauthorized),
            RedisError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            SmtpError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            LetterError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            HashError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            ParseFloatError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            TeraError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            Base64Error(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            DatabaseError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            Infallible(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            TypeHeaderError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            UnknownError(_err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError)
            },
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientResponseError::InternalServerError),
        }
    }
}
