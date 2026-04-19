use crate::{
    application::{
        ports::events_repository::EventsRepository,
        queries::{
            get_event_list_query::GetEventListQuery,
            get_idempotency_query::GetIdempotencyQuery,
            get_metrics_by_service_query::GetMetricsByServiceQuery,
            get_overview_metrics_query::GetOverviewMetricsQuery,
            get_trace_list_query::GetTraceListQuery,
            get_trace_query::GetTraceQuery,
        },
        services::ingest_event_service::IngestEventService,
    },
    config::AppConfig,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HttpState {
    pub config: Arc<AppConfig>,
    pub repository: Arc<dyn EventsRepository>,
    pub ingest_event_service: IngestEventService,
    pub get_event_list_query: GetEventListQuery,
    pub get_trace_query: GetTraceQuery,
    pub get_trace_list_query: GetTraceListQuery,
    pub get_idempotency_query: GetIdempotencyQuery,
    pub get_overview_metrics_query: GetOverviewMetricsQuery,
    pub get_metrics_by_service_query: GetMetricsByServiceQuery,
}