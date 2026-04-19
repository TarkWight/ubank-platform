use crate::{
    application::{
        ports::events_repository::EventsRepository,
        queries::{
            get_idempotency_query::GetIdempotencyQuery,
            get_overview_metrics_query::GetOverviewMetricsQuery,
            get_trace_list_query::GetTraceListQuery,
            get_trace_query::GetTraceQuery,
        },
    },
    config::AppConfig,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct HttpState {
    pub config: Arc<AppConfig>,
    pub repository: Arc<dyn EventsRepository>,
    pub get_trace_query: GetTraceQuery,
    pub get_trace_list_query: GetTraceListQuery,
    pub get_idempotency_query: GetIdempotencyQuery,
    pub get_overview_metrics_query: GetOverviewMetricsQuery,
}