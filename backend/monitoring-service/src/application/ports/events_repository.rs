use async_trait::async_trait;
use serde::Serialize;
use serde_json::Value;
use time::OffsetDateTime;

use crate::{
    domain::monitoring_event::MonitoringEvent,
    shared::error::AppResult,
};

#[derive(Debug, Clone, Serialize)]
pub struct TraceEventView {
    pub id: i64,
    pub trace_id: String,
    pub event_type: String,
    pub event_timestamp: OffsetDateTime,
    pub service: String,
    pub operation: Option<String>,
    pub span_id: Option<String>,
    pub parent_span_id: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i32>,
    pub duration_ms: Option<i64>,
    pub success: Option<bool>,
    pub attempt: Option<i32>,
    pub error_code: Option<String>,
    pub error_type: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverviewMetrics {
    pub total_events: i64,
    pub total_requests: i64,
    pub total_errors: i64,
    pub avg_duration_ms: Option<f64>,
    pub total_retries: i64,
    pub total_circuit_breaker_open: i64,
}

#[async_trait]
pub trait EventsRepository: Send + Sync {
    async fn ping(&self) -> AppResult<()>;

    async fn insert_event(
        &self,
        event: &MonitoringEvent,
        raw_payload: &Value,
    ) -> AppResult<()>;

    async fn get_trace_events(
        &self,
        trace_id: &str,
    ) -> AppResult<Vec<TraceEventView>>;

    async fn get_overview_metrics(&self) -> AppResult<OverviewMetrics>;
}
