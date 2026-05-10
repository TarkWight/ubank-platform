use std::sync::Arc;

use time::OffsetDateTime;

use crate::{
    application::ports::events_repository::{
        EventsRepository,
        MetricsBucket,
        MetricsTimeseriesPointView,
        MetricsTimeseriesQuery,
    },
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct GetMetricsTimeseriesQuery {
    repository: Arc<dyn EventsRepository>,
}

#[derive(Debug, Clone)]
pub struct GetMetricsTimeseriesInput {
    pub bucket: Option<String>,
    pub from: Option<OffsetDateTime>,
    pub to: Option<OffsetDateTime>,
}

impl GetMetricsTimeseriesQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: GetMetricsTimeseriesInput,
    ) -> AppResult<Vec<MetricsTimeseriesPointView>> {
        if let (Some(from), Some(to)) = (input.from, input.to) {
            if from > to {
                return Err(AppError::validation("from must be <= to"));
            }
        }

        let bucket = match input
            .bucket
            .unwrap_or_else(|| "minute".to_string())
            .trim()
            .to_lowercase()
            .as_str()
        {
            "minute" => MetricsBucket::Minute,
            "hour" => MetricsBucket::Hour,
            _ => return Err(AppError::validation("bucket must be minute or hour")),
        };

        self.repository
            .get_metrics_timeseries(MetricsTimeseriesQuery {
                bucket,
                from: input.from,
                to: input.to,
            })
            .await
    }
}