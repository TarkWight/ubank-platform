use std::sync::Arc;

use crate::{
    application::ports::events_repository::{EventsRepository, OverviewMetrics},
    shared::error::AppResult,
};

#[derive(Clone)]
pub struct GetOverviewMetricsQuery {
    repository: Arc<dyn EventsRepository>,
}

impl GetOverviewMetricsQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> AppResult<OverviewMetrics> {
        self.repository.get_overview_metrics().await
    }
}
