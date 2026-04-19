use std::sync::Arc;

use crate::{
    application::ports::events_repository::{EventsRepository, IdempotencyEventView},
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct GetIdempotencyQuery {
    repository: Arc<dyn EventsRepository>,
}

impl GetIdempotencyQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        idempotency_key: &str,
    ) -> AppResult<Vec<IdempotencyEventView>> {
        if idempotency_key.trim().is_empty() {
            return Err(AppError::validation("idempotencyKey is empty"));
        }

        self.repository
            .get_events_by_idempotency_key(idempotency_key)
            .await
    }
}