use std::sync::Arc;

use crate::{
    application::ports::events_repository::{EventsRepository, OperationMetricsView},
    shared::error::AppResult,
};

#[derive(Clone)]
pub struct GetMetricsByOperationQuery {
    repository: Arc<dyn EventsRepository>,
}

impl GetMetricsByOperationQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(&self) -> AppResult<Vec<OperationMetricsView>> {
        self.repository.get_metrics_by_operation().await
    }
}