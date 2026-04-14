use serde::Serialize;

use crate::application::ports::events_repository::{OverviewMetrics, TraceEventView};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: String,
}

#[derive(Debug, Serialize)]
pub struct TraceResponse {
    pub trace_id: String,
    pub events: Vec<TraceEventView>,
}

#[derive(Debug, Serialize)]
pub struct OverviewMetricsResponse {
    pub total_events: i64,
    pub total_requests: i64,
    pub total_errors: i64,
    pub error_rate_percent: f64,
    pub avg_duration_ms: Option<f64>,
    pub total_retries: i64,
    pub total_circuit_breaker_open: i64,
}

impl From<OverviewMetrics> for OverviewMetricsResponse {
    fn from(value: OverviewMetrics) -> Self {
        let error_rate_percent = if value.total_requests == 0 {
            0.0
        } else {
            (value.total_errors as f64 / value.total_requests as f64) * 100.0
        };

        Self {
            total_events: value.total_events,
            total_requests: value.total_requests,
            total_errors: value.total_errors,
            error_rate_percent,
            avg_duration_ms: value.avg_duration_ms,
            total_retries: value.total_retries,
            total_circuit_breaker_open: value.total_circuit_breaker_open,
        }
    }
}
