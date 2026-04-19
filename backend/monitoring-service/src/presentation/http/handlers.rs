use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use serde_json::to_value;

use crate::{
    presentation::http::{
        dto::{
            BatchItemErrorResponse,
            HealthResponse,
            IdempotencyResponse,
            IngestEventsBatchRequest,
            IngestEventsBatchResponse,
            OverviewMetricsResponse,
            TraceListResponse,
            TraceResponse,
            EventListResponse,
        },
        state::HttpState,
    },
    shared::error::{AppError, AppResult},
};

#[derive(Debug, Deserialize)]
pub struct TraceListQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventListQueryParams {
    pub service: Option<String>,
    pub event_type: Option<String>,
    pub trace_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub operation: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub async fn live(
    State(state): State<HttpState>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "OK",
        service: state.config.app_name.clone(),
    })
}

pub async fn ready(
    State(state): State<HttpState>,
) -> AppResult<Json<HealthResponse>> {
    state.repository.ping().await?;

    Ok(Json(HealthResponse {
        status: "OK",
        service: state.config.app_name.clone(),
    }))
}

pub async fn ingest_events_batch(
    State(state): State<HttpState>,
    Json(request): Json<IngestEventsBatchRequest>,
) -> AppResult<Json<IngestEventsBatchResponse>> {
    if request.events.is_empty() {
        return Err(AppError::validation("events batch must not be empty"));
    }

    if request.events.len() > state.config.max_batch_size {
        return Err(AppError::validation(format!(
            "events batch size must be <= {}",
            state.config.max_batch_size
        )));
    }

    let mut accepted_count = 0usize;
    let mut errors = Vec::new();

    for (index, event) in request.events.into_iter().enumerate() {
        let raw_payload = match to_value(&event) {
            Ok(value) => value,
            Err(err) => {
                errors.push(BatchItemErrorResponse {
                    index,
                    message: format!("failed to serialize event to raw payload: {err}"),
                });
                continue;
            }
        };

        match state
            .ingest_event_service
            .ingest(&event, &raw_payload)
            .await
        {
            Ok(()) => {
                accepted_count += 1;
            }
            Err(err) => {
                errors.push(BatchItemErrorResponse {
                    index,
                    message: err.to_string(),
                });
            }
        }
    }

    let rejected_count = errors.len();

    Ok(Json(IngestEventsBatchResponse {
        accepted_count,
        rejected_count,
        errors,
    }))
}

pub async fn get_trace(
    State(state): State<HttpState>,
    Path(trace_id): Path<String>,
) -> AppResult<Json<TraceResponse>> {
    let events = state.get_trace_query.execute(&trace_id).await?;

    if events.is_empty() {
        return Err(AppError::not_found(format!(
            "trace '{}' not found",
            trace_id
        )));
    }

    Ok(Json(TraceResponse::from_trace_events(trace_id, events)))
}

pub async fn get_trace_list(
    State(state): State<HttpState>,
    Query(params): Query<TraceListQueryParams>,
) -> AppResult<Json<TraceListResponse>> {
    let items = state
        .get_trace_list_query
        .execute(params.limit, params.offset)
        .await?;

    Ok(Json(TraceListResponse {
        items: items.into_iter().map(Into::into).collect(),
    }))
}

pub async fn get_idempotency(
    State(state): State<HttpState>,
    Path(idempotency_key): Path<String>,
) -> AppResult<Json<IdempotencyResponse>> {
    let events = state
        .get_idempotency_query
        .execute(&idempotency_key)
        .await?;

    if events.is_empty() {
        return Err(AppError::not_found(format!(
            "idempotency key '{}' not found",
            idempotency_key
        )));
    }

    Ok(Json(IdempotencyResponse::from_events(
        idempotency_key,
        events,
    )))
}

pub async fn get_overview_metrics(
    State(state): State<HttpState>,
) -> AppResult<Json<OverviewMetricsResponse>> {
    let metrics = state.get_overview_metrics_query.execute().await?;
    Ok(Json(metrics.into()))
}

pub async fn get_event_list(
    State(state): State<HttpState>,
    Query(params): Query<EventListQueryParams>,
) -> AppResult<Json<EventListResponse>> {
    let from = parse_optional_rfc3339(params.from.as_deref(), "from")?;
    let to = parse_optional_rfc3339(params.to.as_deref(), "to")?;

    let limit = params.limit.unwrap_or(50);
    let offset = params.offset.unwrap_or(0);

    let items = state
        .get_event_list_query
        .execute(crate::application::queries::get_event_list_query::GetEventListInput {
            service: params.service,
            event_type: params.event_type,
            trace_id: params.trace_id,
            idempotency_key: params.idempotency_key,
            operation: params.operation,
            from,
            to,
            limit: Some(limit),
            offset: Some(offset),
        })
        .await?;

    Ok(Json(EventListResponse {
        items: items.into_iter().map(Into::into).collect(),
        limit,
        offset,
    }))
}

fn parse_optional_rfc3339(
    value: Option<&str>,
    field_name: &str,
) -> AppResult<Option<time::OffsetDateTime>> {
    match value {
        Some(raw) => {
            let trimmed = raw.trim();
            if trimmed.is_empty() {
                return Ok(None);
            }

            let parsed = time::OffsetDateTime::parse(
                trimmed,
                &time::format_description::well_known::Rfc3339,
            )
                .map_err(|_| AppError::validation(format!(
                    "{field_name} must be a valid RFC3339 datetime"
                )))?;

            Ok(Some(parsed))
        }
        None => Ok(None),
    }
}