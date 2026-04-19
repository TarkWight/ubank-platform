use async_trait::async_trait;
use serde_json::Value;
use sqlx::{postgres::PgPool, Row};
use time::OffsetDateTime;

use crate::{
    application::ports::events_repository::{EventsRepository, OverviewMetrics, TraceEventView},
    domain::monitoring_event::MonitoringEvent,
    shared::error::AppResult,
};

#[derive(Clone)]
pub struct PostgresEventsRepository {
    pool: PgPool,
}

impl PostgresEventsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EventsRepository for PostgresEventsRepository {
    async fn ping(&self) -> AppResult<()> {
        sqlx::query("select 1")
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn insert_event(
        &self,
        event: &MonitoringEvent,
        raw_payload: &Value,
    ) -> AppResult<()> {
        let error_code = event.error.as_ref().and_then(|x| x.code.clone());
        let error_type = event.error.as_ref().and_then(|x| x.error_type.clone());
        let error_message = event.error.as_ref().and_then(|x| x.message.clone());

        sqlx::query(
            r#"
            insert into monitoring_events (
                trace_id,
                event_type,
                event_timestamp,
                service,
                operation,
                span_id,
                parent_span_id,
                method,
                path,
                status,
                duration_ms,
                success,
                attempt,
                error_code,
                error_type,
                error_message,
                raw_payload
            ) values (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17
            )
            "#,
        )
        .bind(&event.trace_id)
        .bind(event.event_type.as_db_value())
        .bind(event.timestamp)
        .bind(&event.service)
        .bind(&event.operation)
        .bind(&event.span_id)
        .bind(&event.parent_span_id)
        .bind(&event.method)
        .bind(&event.path)
        .bind(event.status)
        .bind(event.duration_ms)
        .bind(event.success)
        .bind(event.attempt)
        .bind(error_code)
        .bind(error_type)
        .bind(error_message)
        .bind(raw_payload)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_trace_events(&self, trace_id: &str) -> AppResult<Vec<TraceEventView>> {
        let rows = sqlx::query(
            r#"
        select
            id,
            trace_id,
            event_type,
            event_timestamp,
            service,
            operation,
            span_id,
            parent_span_id,
            method,
            path,
            status,
            duration_ms,
            success,
            attempt,
            error_code,
            error_type,
            error_message
        from monitoring_events
        where trace_id = $1
        order by event_timestamp asc, id asc
        "#,
        )
            .bind(trace_id)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| TraceEventView {
                id: row.get("id"),
                trace_id: row.get("trace_id"),
                event_type: row.get("event_type"),
                event_timestamp: row.get("event_timestamp"),
                service: row.get("service"),
                operation: row.get("operation"),
                span_id: row.get("span_id"),
                parent_span_id: row.get("parent_span_id"),
                method: row.get("method"),
                path: row.get("path"),
                status: row.get("status"),
                duration_ms: row.get("duration_ms"),
                success: row.get("success"),
                attempt: row.get("attempt"),
                error_code: row.get("error_code"),
                error_type: row.get("error_type"),
                error_message: row.get("error_message"),
            })
            .collect())
    }

    async fn get_overview_metrics(&self) -> AppResult<OverviewMetrics> {
        let row = sqlx::query(
            r#"
            select
                count(*)::bigint as total_events,
                count(*) filter (where event_type in ('RESPONSE', 'ERROR'))::bigint as total_requests,
                count(*) filter (where event_type = 'ERROR')::bigint as total_errors,
                avg(duration_ms) filter (where event_type in ('RESPONSE', 'ERROR'))::double precision as avg_duration_ms,
                count(*) filter (where event_type = 'RETRY')::bigint as total_retries,
                count(*) filter (where event_type = 'CIRCUIT_BREAKER_OPEN')::bigint as total_circuit_breaker_open
            from monitoring_events
            "#,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(OverviewMetrics {
            total_events: row.get("total_events"),
            total_requests: row.get("total_requests"),
            total_errors: row.get("total_errors"),
            avg_duration_ms: row.get("avg_duration_ms"),
            total_retries: row.get("total_retries"),
            total_circuit_breaker_open: row.get("total_circuit_breaker_open"),
        })
    }
}
