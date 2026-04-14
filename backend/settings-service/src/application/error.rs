use thiserror::Error;

use crate::infrastructure::persistence::error::RepositoryError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Forbidden")]
    Forbidden,

    #[error("Invalid locale")]
    InvalidLocale,

    #[error("Invalid account id")]
    InvalidAccountId,

    #[error("Persistence failure")]
    Persistence(#[from] RepositoryError),
}