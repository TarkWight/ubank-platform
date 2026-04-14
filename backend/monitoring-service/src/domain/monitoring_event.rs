use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MonitoringEventType {
    Request,
    Response,
    Error,
    Retry,
    CircuitBreakerOpen,
    CircuitBreakerClose,
}

impl MonitoringEventType {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Request => "REQUEST",
            Self::Response => "RESPONSE",
            Self::Error => "ERROR",
            Self::Retry => "RETRY",
            Self::CircuitBreakerOpen => "CIRCUIT_BREAKER_OPEN",
            Self::CircuitBreakerClose => "CIRCUIT_BREAKER_CLOSE",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringError {
    pub code: Option<String>,
    #[serde(rename = "type")]
    pub error_type: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitoringEvent {
    pub trace_id: String,
    pub event_type: MonitoringEventType,

    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,

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
    pub error: Option<MonitoringError>,
}

impl MonitoringEvent {
    pub fn validate(&self) -> Result<(), String> {
        if self.trace_id.trim().is_empty() {
            return Err("traceId is empty".to_string());
        }

        if self.service.trim().is_empty() {
            return Err("service is empty".to_string());
        }

        if let Some(duration_ms) = self.duration_ms {
            if duration_ms < 0 {
                return Err("durationMs must be >= 0".to_string());
            }
        }

        if let Some(attempt) = self.attempt {
            if attempt <= 0 {
                return Err("attempt must be > 0".to_string());
            }
        }

        Ok(())
    }
}
