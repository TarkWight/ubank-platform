use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Missing authorization header")]
    MissingAuthorizationHeader,

    #[error("Invalid authorization header")]
    InvalidAuthorizationHeader,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Expired token")]
    ExpiredToken,

    #[error("Invalid token subject")]
    InvalidSubject,

    #[error("Unsupported role claim")]
    UnsupportedRoleClaim,
}

#[derive(Debug, Error)]
pub enum AuthInitError {
    #[error("Invalid JWT base64 secret")]
    InvalidJwtSecret,
}