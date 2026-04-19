use async_trait::async_trait;
use serde_json::Value;
use sqlx::{postgres::PgPool, Row};

use crate::{
    application::ports::events_repository::{
        EventListItemView,
        EventListQuery,
        EventsRepository,
        IdempotencyEventView,
        OverviewMetrics,
        TraceEventView,
        TraceListItemView,
    },
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
                idempotency_key,
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
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18
            )
            "#,
        )
            .bind(&event.trace_id)
            .bind(&event.idempotency_key)
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

    async fn get_trace_list(&self, limit: i64, offset: i64) -> AppResult<Vec<TraceListItemView>> {
        let rows = sqlx::query(
            r#"
            select
                trace_id,
                count(*)::bigint as event_count,
                min(event_timestamp) as started_at,
                max(event_timestamp) as finished_at,
                array_agg(distinct service) as services,
                bool_or(event_type = 'ERROR') as has_error
            from monitoring_events
            group by trace_id
            order by max(event_timestamp) desc
            limit $1 offset $2
            "#,
        )
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| TraceListItemView {
                trace_id: row.get("trace_id"),
                event_count: row.get("event_count"),
                started_at: row.get("started_at"),
                finished_at: row.get("finished_at"),
                services: row.get("services"),
                has_error: row.get("has_error"),
            })
            .collect())
    }

    async fn get_trace_events(&self, trace_id: &str) -> AppResult<Vec<TraceEventView>> {
        let rows = sqlx::query(
            r#"
            select
                id,
                trace_id,
                idempotency_key,
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
                idempotency_key: row.get("idempotency_key"),
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

    async fn get_events_by_idempotency_key(
        &self,
        idempotency_key: &str,
    ) -> AppResult<Vec<IdempotencyEventView>> {
        let rows = sqlx::query(
            r#"
            select
                id,
                trace_id,
                idempotency_key,
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
            where idempotency_key = $1
            order by event_timestamp asc, id asc
            "#,
        )
            .bind(idempotency_key)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| IdempotencyEventView {
                id: row.get("id"),
                trace_id: row.get("trace_id"),
                idempotency_key: row.get("idempotency_key"),
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

    async fn get_event_list(&self, query: EventListQuery) -> AppResult<Vec<EventListItemView>> {
        let rows = sqlx::query(
            r#"
            select
                id,
                trace_id,
                idempotency_key,
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
            where ($1::varchar is null or service = $1)
              and ($2::varchar is null or event_type = $2)
              and ($3::varchar is null or trace_id = $3)
              and ($4::varchar is null or idempotency_key = $4)
              and ($5::varchar is null or operation = $5)
              and ($6::timestamptz is null or event_timestamp >= $6)
              and ($7::timestamptz is null or event_timestamp <= $7)
            order by event_timestamp desc, id desc
            limit $8 offset $9
            "#,
        )
            .bind(query.service)
            .bind(query.event_type)
            .bind(query.trace_id)
            .bind(query.idempotency_key)
            .bind(query.operation)
            .bind(query.from)
            .bind(query.to)
            .bind(query.limit)
            .bind(query.offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| EventListItemView {
                id: row.get("id"),
                trace_id: row.get("trace_id"),
                idempotency_key: row.get("idempotency_key"),
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
                count(*) filter (where event_type = 'CIRCUIT_BREAKER_OPEN')::bigint as total_circuit_breaker_open,
                count(*) filter (where event_type = 'IDEMPOTENCY_REPLAY')::bigint as total_idempotency_replays,
                count(*) filter (where event_type = 'IDEMPOTENCY_IN_PROGRESS')::bigint as total_idempotency_in_progress,
                count(*) filter (where event_type = 'IDEMPOTENCY_CONFLICT')::bigint as total_idempotency_conflicts
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
            total_idempotency_replays: row.get("total_idempotency_replays"),
            total_idempotency_in_progress: row.get("total_idempotency_in_progress"),
            total_idempotency_conflicts: row.get("total_idempotency_conflicts"),
        })
    }
}