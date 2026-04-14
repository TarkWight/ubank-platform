mod application;
mod bootstrap;
mod domain;
mod infrastructure;
mod presentation;

use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{
    application::services::settings_service::SettingsApplicationService,
    bootstrap::config::Config,
    infrastructure::{
        auth::{jwt_validator::JwtValidator, middleware::AuthState},
        persistence::{
            hidden_accounts_repository_pg::HiddenAccountsRepositoryPg,
            settings_repository_pg::SettingsRepositoryPg,
        },
    },
    presentation::http::{handlers::HttpState, routes::build_router},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env()?;
    tracing::info!("JWT secret length: {}", config.jwt_secret.len());

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    let settings_repo = Arc::new(SettingsRepositoryPg::new(pool.clone()));
    let hidden_repo = Arc::new(HiddenAccountsRepositoryPg::new(pool));

    let settings_service = Arc::new(SettingsApplicationService::new(
        settings_repo,
        hidden_repo,
        config.default_locale.clone(),
    ));

    let auth_state = AuthState {
        jwt_validator: JwtValidator::new(&config.jwt_secret)?,
    };

    let router = build_router(
        HttpState { settings_service },
        auth_state,
    );

    let listener = tokio::net::TcpListener::bind(config.server_addr).await?;
    tracing::info!("settings-service listening on {}", config.server_addr);

    axum::serve(listener, router).await?;

    Ok(())
}