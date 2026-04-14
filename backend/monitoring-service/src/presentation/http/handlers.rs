use axum::{
    extract::{Path, State},
    Json,
};

use crate::{
    presentation::http::{
        dto::{HealthResponse, OverviewMetricsResponse, TraceResponse},
        state::HttpState,
    },
    shared::error::{AppError, AppResult},
};

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
        return Err(AppError::not_found(format!("trace '{}' not found", trace_id)));
    }

    Ok(Json(TraceResponse::from_trace_events(trace_id, events)))
}

pub async fn get_overview_metrics(
    State(state): State<HttpState>,
) -> AppResult<Json<OverviewMetricsResponse>> {
    let metrics = state.get_overview_metrics_query.execute().await?;
    Ok(Json(metrics.into()))
}