use serde::Serialize;
use time::OffsetDateTime;

use crate::application::ports::events_repository::{OverviewMetrics, TraceEventView};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub service: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceErrorResponse {
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceEventResponse {
    pub id: i64,
    pub timestamp: String,
    pub service: String,
    pub operation: Option<String>,
    pub event_type: String,
    pub span_id: Option<String>,
    pub parent_span_id: Option<String>,
    pub method: Option<String>,
    pub path: Option<String>,
    pub status: Option<i32>,
    pub duration_ms: Option<i64>,
    pub success: Option<bool>,
    pub attempt: Option<i32>,
    pub error: Option<TraceErrorResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TraceResponse {
    pub trace_id: String,
    pub event_count: usize,
    pub started_at: Option<String>,
    pub finished_at: Option<String>,
    pub duration_ms: Option<i64>,
    pub events: Vec<TraceEventResponse>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
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

impl From<TraceEventView> for TraceEventResponse {
    fn from(value: TraceEventView) -> Self {
        Self {
            id: value.id,
            timestamp: format_rfc3339(value.event_timestamp),
            service: value.service,
            operation: value.operation,
            event_type: value.event_type,
            span_id: value.span_id,
            parent_span_id: value.parent_span_id,
            method: value.method,
            path: value.path,
            status: value.status,
            duration_ms: value.duration_ms,
            success: value.success,
            attempt: value.attempt,
            error: if value.error_code.is_some()
                || value.error_type.is_some()
                || value.error_message.is_some()
            {
                Some(TraceErrorResponse {
                    code: value.error_code,
                    error_type: value.error_type,
                    message: value.error_message,
                })
            } else {
                None
            },
        }
    }
}

impl TraceResponse {
    pub fn from_trace_events(trace_id: String, events: Vec<TraceEventView>) -> Self {
        let event_count = events.len();

        let started_at_raw = events.first().map(|x| x.event_timestamp);
        let finished_at_raw = events.last().map(|x| x.event_timestamp);

        let duration_ms = match (started_at_raw, finished_at_raw) {
            (Some(start), Some(finish)) => {
                let diff = finish - start;
                i64::try_from(diff.whole_milliseconds()).ok()
            }
            _ => None,
        };

        let mapped_events = events
            .into_iter()
            .map(TraceEventResponse::from)
            .collect::<Vec<_>>();

        Self {
            trace_id,
            event_count,
            started_at: started_at_raw.map(format_rfc3339),
            finished_at: finished_at_raw.map(format_rfc3339),
            duration_ms,
            events: mapped_events,
        }
    }
}

fn format_rfc3339(value: OffsetDateTime) -> String {
    value
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| value.to_string())
}