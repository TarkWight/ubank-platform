use std::sync::Arc;

use crate::{
    application::ports::events_repository::{EventsRepository, ServiceMetricsView},
    shared::error::AppResult,
};

#[derive(Clone)]
pub struct GetMetricsByServiceQuery {
    repository: Arc<dyn EventsRepository>,
}

impl GetMetricsByServiceQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> AppResult<Vec<ServiceMetricsView>> {
        self.repository.get_metrics_by_service().await
    }
}