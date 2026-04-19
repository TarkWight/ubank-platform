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
    pub idempotency_key: Option<String>,
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
pub struct IdempotencyEventView {
    pub id: i64,
    pub trace_id: String,
    pub idempotency_key: String,
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
pub struct EventListItemView {
    pub id: i64,
    pub trace_id: String,
    pub idempotency_key: Option<String>,
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

#[derive(Debug, Clone)]
pub struct EventListQuery {
    pub service: Option<String>,
    pub event_type: Option<String>,
    pub trace_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub operation: Option<String>,
    pub from: Option<OffsetDateTime>,
    pub to: Option<OffsetDateTime>,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OverviewMetrics {
    pub total_events: i64,
    pub total_requests: i64,
    pub total_errors: i64,
    pub avg_duration_ms: Option<f64>,
    pub total_retries: i64,
    pub total_circuit_breaker_open: i64,
    pub total_idempotency_replays: i64,
    pub total_idempotency_in_progress: i64,
    pub total_idempotency_conflicts: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TraceListItemView {
    pub trace_id: String,
    pub event_count: i64,
    pub started_at: OffsetDateTime,
    pub finished_at: OffsetDateTime,
    pub services: Vec<String>,
    pub has_error: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct ServiceMetricsView {
    pub service: String,
    pub total_events: i64,
    pub total_requests: i64,
    pub total_errors: i64,
    pub avg_duration_ms: Option<f64>,
    pub total_retries: i64,
    pub total_circuit_breaker_open: i64,
    pub total_idempotency_replays: i64,
    pub total_idempotency_in_progress: i64,
    pub total_idempotency_conflicts: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct OperationMetricsView {
    pub service: String,
    pub operation: String,
    pub total_events: i64,
    pub total_requests: i64,
    pub total_errors: i64,
    pub avg_duration_ms: Option<f64>,
    pub total_retries: i64,
    pub total_circuit_breaker_open: i64,
    pub total_idempotency_replays: i64,
    pub total_idempotency_in_progress: i64,
    pub total_idempotency_conflicts: i64,
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

    async fn get_events_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> AppResult<Vec<IdempotencyEventView>>;

    async fn get_event_list(
        &self,
        query: EventListQuery,
    ) -> AppResult<Vec<EventListItemView>>;

    async fn get_trace_list(
        &self,
        limit: i64,
        offset: i64,
    ) -> AppResult<Vec<TraceListItemView>>;

    async fn get_overview_metrics(&self) -> AppResult<OverviewMetrics>;

    async fn get_metrics_by_service(&self) -> AppResult<Vec<ServiceMetricsView>>;

    async fn get_metrics_by_operation(&self) -> AppResult<Vec<OperationMetricsView>>;
}