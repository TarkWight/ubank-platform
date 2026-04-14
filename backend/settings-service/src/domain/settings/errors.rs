use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid locale")]
    InvalidLocale,
}