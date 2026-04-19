use std::sync::Arc;

use crate::{
    application::ports::events_repository::{EventsRepository, TraceEventView},
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct GetTraceQuery {
    repository: Arc<dyn EventsRepository>,
}

impl GetTraceQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, trace_id: &str) -> AppResult<Vec<TraceEventView>> {
        if trace_id.trim().is_empty() {
            return Err(AppError::validation("trace_id is empty"));
        }

        self.repository.get_trace_events(trace_id).await
    }
}