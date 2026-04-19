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
    IdempotencyReplay,
    IdempotencyInProgress,
    IdempotencyConflict,
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
            Self::IdempotencyReplay => "IDEMPOTENCY_REPLAY",
            Self::IdempotencyInProgress => "IDEMPOTENCY_IN_PROGRESS",
            Self::IdempotencyConflict => "IDEMPOTENCY_CONFLICT",
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
    pub idempotency_key: Option<String>,
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

        if let Some(operation) = &self.operation {
            if operation.trim().is_empty() {
                return Err("operation must not be blank".to_string());
            }
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

        match self.event_type {
            MonitoringEventType::IdempotencyReplay
            | MonitoringEventType::IdempotencyInProgress
            | MonitoringEventType::IdempotencyConflict => {
                match &self.idempotency_key {
                    Some(key) if !key.trim().is_empty() => {}
                    _ => {
                        return Err(
                            "idempotencyKey is required for idempotency events".to_string(),
                        )
                    }
                }
            }
            MonitoringEventType::Retry => {
                if let Some(method) = &self.method {
                    let upper = method.to_uppercase();
                    let is_write_like =
                        matches!(upper.as_str(), "POST" | "PUT" | "PATCH" | "DELETE");

                    if is_write_like {
                        match &self.idempotency_key {
                            Some(key) if !key.trim().is_empty() => {}
                            _ => {
                                return Err(
                                    "idempotencyKey is required for RETRY of write-like requests"
                                        .to_string(),
                                )
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}