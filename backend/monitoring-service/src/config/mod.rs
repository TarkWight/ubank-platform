use std::env;

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub app_name: String,
    pub http_addr: String,
    pub cors_allow_origin: String,
    pub postgres_url: String,
    pub rabbitmq_url: String,
    pub rabbitmq_exchange: String,
    pub rabbitmq_queue: String,
    pub rabbitmq_dlx: String,
    pub rabbitmq_dlq: String,
    pub rabbitmq_routing_key: String,
    pub rabbitmq_dlq_routing_key: String,
    pub consumer_tag: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            app_name: env::var("APP_NAME").unwrap_or_else(|_| "monitoring-service".to_string()),
            http_addr: env::var("HTTP_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            cors_allow_origin: env::var("CORS_ALLOW_ORIGIN").unwrap_or_else(|_| "*".to_string()),
            postgres_url: env::var("POSTGRES_URL").expect("POSTGRES_URL is required"),
            rabbitmq_url: env::var("RABBITMQ_URL").expect("RABBITMQ_URL is required"),
            rabbitmq_exchange: env::var("RABBITMQ_EXCHANGE").unwrap_or_else(|_| "monitoring.exchange".to_string()),
            rabbitmq_queue: env::var("RABBITMQ_QUEUE").unwrap_or_else(|_| "monitoring.queue".to_string()),
            rabbitmq_dlx: env::var("RABBITMQ_DLX").unwrap_or_else(|_| "monitoring.dlx".to_string()),
            rabbitmq_dlq: env::var("RABBITMQ_DLQ").unwrap_or_else(|_| "monitoring.queue.dlq".to_string()),
            rabbitmq_routing_key: env::var("RABBITMQ_ROUTING_KEY").unwrap_or_else(|_| "monitoring.event".to_string()),
            rabbitmq_dlq_routing_key: env::var("RABBITMQ_DLQ_ROUTING_KEY").unwrap_or_else(|_| "monitoring.dead".to_string()),
            consumer_tag: env::var("CONSUMER_TAG").unwrap_or_else(|_| "monitoring-service-consumer".to_string()),
        }
    }
}
