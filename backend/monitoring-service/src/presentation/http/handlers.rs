use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;

use crate::{
    presentation::http::{
        dto::{
            HealthResponse,
            IdempotencyResponse,
            OverviewMetricsResponse,
            TraceListResponse,
            TraceResponse,
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