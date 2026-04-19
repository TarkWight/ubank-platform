use std::sync::Arc;

use crate::{
    application::ports::events_repository::{EventsRepository, TraceListItemView},
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct GetTraceListQuery {
    repository: Arc<dyn EventsRepository>,
}

impl GetTraceListQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> AppResult<Vec<TraceListItemView>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        if limit <= 0 {
            return Err(AppError::validation("limit must be > 0"));
        }

        if limit > 200 {
            return Err(AppError::validation("limit must be <= 200"));
        }

        if offset < 0 {
            return Err(AppError::validation("offset must be >= 0"));
        }

        self.repository.get_trace_list(limit, offset).await
    }
}