use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use time::OffsetDateTime;
use thiserror::Error;

use crate::{
    application::error::ApplicationError,
    infrastructure::auth::error::AuthError,
};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Error)]
pub enum HttpError {
    #[error(transparent)]
    Auth(#[from] AuthError),

    #[error(transparent)]
    Application(#[from] ApplicationError),

    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            HttpError::Auth(AuthError::MissingAuthorizationHeader)
            | HttpError::Auth(AuthError::InvalidAuthorizationHeader)
            | HttpError::Auth(AuthError::InvalidToken)
            | HttpError::Auth(AuthError::ExpiredToken)
            | HttpError::Auth(AuthError::InvalidSubject)
            | HttpError::Auth(AuthError::UnsupportedRoleClaim) => {
                (StatusCode::UNAUTHORIZED, "Unauthorized".to_string())
            }

            HttpError::Application(ApplicationError::Forbidden) => {
                (StatusCode::FORBIDDEN, "Forbidden".to_string())
            }

            HttpError::Application(ApplicationError::InvalidLocale) => {
                (StatusCode::BAD_REQUEST, "Invalid locale".to_string())
            }

            HttpError::Application(ApplicationError::InvalidAccountId) => {
                (StatusCode::BAD_REQUEST, "Invalid account id".to_string())
            }

            HttpError::Application(ApplicationError::Persistence(_))
            | HttpError::Internal => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string())
            }
        };

        let body = Json(ErrorResponse {
            message,
            timestamp: OffsetDateTime::now_utc(),
        });

        (status, body).into_response()
    }
}