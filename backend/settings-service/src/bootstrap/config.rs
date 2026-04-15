use std::{env, net::SocketAddr, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Config {
    pub server_addr: SocketAddr,
    pub database_url: String,
    pub jwt_secret: String,
    pub default_locale: String,
    pub fault_injection_enabled: bool,
    pub base_error_rate: f64,
    pub even_minute_error_rate: f64,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("DATABASE_URL is required")]
    MissingDatabaseUrl,

    #[error("JWT_SECRET is required")]
    MissingJwtSecret,

    #[error("Invalid SERVER_PORT")]
    InvalidServerPort(#[source] ParseIntError),

    #[error("Invalid BASE_ERROR_RATE")]
    InvalidBaseErrorRate,

    #[error("Invalid EVEN_MINUTE_ERROR_RATE")]
    InvalidEvenMinuteErrorRate,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let port: u16 = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "8084".to_string())
            .parse()
            .map_err(ConfigError::InvalidServerPort)?;

        let database_url =
            env::var("DATABASE_URL").map_err(|_| ConfigError::MissingDatabaseUrl)?;

        let jwt_secret =
            env::var("JWT_SECRET").map_err(|_| ConfigError::MissingJwtSecret)?;

        let default_locale = env::var("DEFAULT_LOCALE").unwrap_or_else(|_| "rus".to_string());

        let fault_injection_enabled = env::var("FAULT_INJECTION_ENABLED")
            .unwrap_or_else(|_| "false".to_string())
            .parse()
            .unwrap_or(false);

        let base_error_rate = env::var("BASE_ERROR_RATE")
            .unwrap_or_else(|_| "0.3".to_string())
            .parse::<f64>()
            .map_err(|_| ConfigError::InvalidBaseErrorRate)?;

        let even_minute_error_rate = env::var("EVEN_MINUTE_ERROR_RATE")
            .unwrap_or_else(|_| "0.7".to_string())
            .parse::<f64>()
            .map_err(|_| ConfigError::InvalidEvenMinuteErrorRate)?;

        Ok(Self {
            server_addr: SocketAddr::from(([127, 0, 0, 1], port)),
            database_url,
            jwt_secret,
            default_locale,
            fault_injection_enabled,
            base_error_rate,
            even_minute_error_rate,
        })
    }
}