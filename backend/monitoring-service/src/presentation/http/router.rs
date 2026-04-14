use axum::{routing::get, Router};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::presentation::http::{handlers, state::HttpState};

pub fn create_router(state: HttpState) -> Router {
    let cors = if state.config.cors_allow_origin == "*" {
        CorsLayer::new().allow_origin(Any).allow_methods(Any).allow_headers(Any)
    } else {
        CorsLayer::new().allow_origin(
            state
                .config
                .cors_allow_origin
                .parse::<axum::http::HeaderValue>()
                .expect("invalid CORS_ALLOW_ORIGIN"),
        )
    };

    Router::new()
        .route("/health/live", get(handlers::live))
        .route("/health/ready", get(handlers::ready))
        .route("/api/v1/traces/{trace_id}", get(handlers::get_trace))
        .route("/api/v1/metrics/overview", get(handlers::get_overview_metrics))
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}
