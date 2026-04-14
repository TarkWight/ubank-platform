use thiserror::Error;

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}