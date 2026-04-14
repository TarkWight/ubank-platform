use std::sync::Arc;

use serde_json::Value;

use crate::{
    application::ports::events_repository::EventsRepository,
    domain::monitoring_event::MonitoringEvent,
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct IngestEventService {
    repository: Arc<dyn EventsRepository>,
}

impl IngestEventService {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn ingest(
        &self,
        event: &MonitoringEvent,
        raw_payload: &Value,
    ) -> AppResult<()> {
        event.validate().map_err(AppError::validation)?;
        self.repository.insert_event(event, raw_payload).await
    }
}
