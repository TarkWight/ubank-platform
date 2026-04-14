use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("validation error: {0}")]
    Validation(String),

    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("infrastructure error: {0}")]
    Infrastructure(String),

    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl AppError {
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation(message.into())
    }

    pub fn not_found(message: impl Into<String>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn infrastructure(message: impl Into<String>) -> Self {
        Self::Infrastructure(message.into())
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::Internal(message.into())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_code, message, server_side) = match self {
            AppError::Validation(message) => (StatusCode::BAD_REQUEST, "validation_error", message, false),
            AppError::NotFound(message) => (StatusCode::NOT_FOUND, "not_found", message, false),
            AppError::Conflict(message) => (StatusCode::CONFLICT, "conflict", message, false),
            AppError::Infrastructure(message) => (StatusCode::SERVICE_UNAVAILABLE, "infrastructure_error", message, true),
            AppError::Internal(message) => (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", message, true),
        };

        if server_side {
            tracing::error!(status = %status, error = error_code, message = %message, "request failed");
        } else {
            tracing::warn!(status = %status, error = error_code, message = %message, "request failed");
        }

        (
            status,
            Json(ErrorResponse {
                error: error_code.to_string(),
                message,
            }),
        ).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self::Infrastructure(value.to_string())
    }
}

impl From<lapin::Error> for AppError {
    fn from(value: lapin::Error) -> Self {
        Self::Infrastructure(value.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        Self::Validation(value.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        Self::Infrastructure(value.to_string())
    }
}
