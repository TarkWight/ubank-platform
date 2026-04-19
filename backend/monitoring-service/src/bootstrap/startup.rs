use std::sync::Arc;

use axum::serve;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing::{error, info};

use crate::{
    application::{
        ports::events_repository::EventsRepository,
        queries::{
            get_idempotency_query::GetIdempotencyQuery,
            get_overview_metrics_query::GetOverviewMetricsQuery,
            get_trace_query::GetTraceQuery,
            get_trace_list_query::GetTraceListQuery,
        },
        services::ingest_event_service::IngestEventService,
    },
    config::AppConfig,
    infrastructure::{
        messaging::rabbitmq_consumer::RabbitMqConsumer,
        persistence::postgres_events_repository::PostgresEventsRepository,
    },
    presentation::http::{router::create_router, state::HttpState},
    shared::error::{AppError, AppResult},
};

pub async fn run() -> AppResult<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    let config = Arc::new(AppConfig::from_env());

    let postgres_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.postgres_url)
        .await?;

    let postgres_repository = Arc::new(PostgresEventsRepository::new(postgres_pool));
    let repository: Arc<dyn EventsRepository> = postgres_repository;

    let ingest_event_service = IngestEventService::new(repository.clone());
    let get_trace_query = GetTraceQuery::new(repository.clone());
    let get_trace_list_query = GetTraceListQuery::new(repository.clone());
    let get_idempotency_query = GetIdempotencyQuery::new(repository.clone());
    let get_overview_metrics_query = GetOverviewMetricsQuery::new(repository.clone());

    let consumer = RabbitMqConsumer::new(config.clone(), ingest_event_service.clone());
    tokio::spawn(async move {
        if let Err(err) = consumer.run().await {
            error!(error = %err, "rabbitmq consumer stopped");
        }
    });

    let http_state = HttpState {
        config: config.clone(),
        repository,
        ingest_event_service,
        get_trace_query,
        get_trace_list_query,
        get_idempotency_query,
        get_overview_metrics_query,
    };

    let app = create_router(http_state);
    let listener = TcpListener::bind(&config.http_addr)
        .await
        .map_err(|e| AppError::infrastructure(e.to_string()))?;

    info!(addr = %config.http_addr, app = %config.app_name, "http server started");

    serve(listener, app)
        .await
        .map_err(|e| AppError::infrastructure(e.to_string()))?;

    Ok(())
}