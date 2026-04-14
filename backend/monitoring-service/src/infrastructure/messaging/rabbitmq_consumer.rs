use std::sync::Arc;

use futures_lite::stream::StreamExt;
use lapin::{
    options::*,
    types::{AMQPValue, FieldTable, LongString, ShortString},
    BasicProperties, Channel, Connection, ConnectionProperties, ExchangeKind,
};
use serde_json::Value;
use tracing::{error, info, warn};

use crate::{
    application::services::ingest_event_service::IngestEventService,
    config::AppConfig,
    domain::monitoring_event::MonitoringEvent,
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct RabbitMqConsumer {
    config: Arc<AppConfig>,
    ingest_service: IngestEventService,
}

impl RabbitMqConsumer {
    pub fn new(config: Arc<AppConfig>, ingest_service: IngestEventService) -> Self {
        Self {
            config,
            ingest_service,
        }
    }

    pub async fn run(&self) -> AppResult<()> {
        let connection = Connection::connect(
            &self.config.rabbitmq_url,
            ConnectionProperties::default(),
        )
            .await?;

        let channel = connection.create_channel().await?;
        self.declare_topology(&channel).await?;

        let mut consumer = channel
            .basic_consume(
                self.queue_name(),
                self.consumer_tag(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await?;

        info!(queue = %self.config.rabbitmq_queue, "rabbitmq consumer started");

        while let Some(delivery_result) = consumer.next().await {
            match delivery_result {
                Ok(delivery) => {
                    let payload = delivery.data.clone();

                    match self.process_message(&payload).await {
                        Ok(()) => {
                            delivery.ack(BasicAckOptions::default()).await?;
                        }
                        Err(MessageProcessingError::InvalidMessage(reason)) => {
                            warn!(reason = %reason, "invalid monitoring message, sending to dlq");

                            self.publish_to_dlq(&channel, &payload).await?;
                            delivery.ack(BasicAckOptions::default()).await?;
                        }
                        Err(MessageProcessingError::TemporaryFailure(err)) => {
                            error!(error = %err, "temporary processing failure, message requeued");

                            delivery
                                .nack(BasicNackOptions {
                                    multiple: false,
                                    requeue: true,
                                })
                                .await?;
                        }
                    }
                }
                Err(err) => {
                    error!(error = %err, "consumer delivery error");
                }
            }
        }

        Ok(())
    }

    async fn declare_topology(&self, channel: &Channel) -> AppResult<()> {
        channel
            .exchange_declare(
                self.exchange_name(),
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .exchange_declare(
                self.dlx_name(),
                ExchangeKind::Direct,
                ExchangeDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        let mut queue_args = FieldTable::default();
        queue_args.insert(
            ShortString::from("x-dead-letter-exchange"),
            AMQPValue::LongString(LongString::from(self.config.rabbitmq_dlx.clone())),
        );
        queue_args.insert(
            ShortString::from("x-dead-letter-routing-key"),
            AMQPValue::LongString(LongString::from(
                self.config.rabbitmq_dlq_routing_key.clone(),
            )),
        );

        channel
            .queue_declare(
                self.queue_name(),
                QueueDeclareOptions::default(),
                queue_args,
            )
            .await?;

        channel
            .queue_declare(
                self.dlq_name(),
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                self.queue_name(),
                self.exchange_name(),
                self.routing_key(),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        channel
            .queue_bind(
                self.dlq_name(),
                self.dlx_name(),
                self.dlq_routing_key(),
                QueueBindOptions::default(),
                FieldTable::default(),
            )
            .await?;

        Ok(())
    }

    async fn process_message(&self, payload: &[u8]) -> Result<(), MessageProcessingError> {
        let raw_payload: Value = serde_json::from_slice(payload)
            .map_err(|e| MessageProcessingError::InvalidMessage(format!("invalid json: {e}")))?;

        let event: MonitoringEvent = serde_json::from_value(raw_payload.clone())
            .map_err(|e| MessageProcessingError::InvalidMessage(format!("invalid schema: {e}")))?;

        self.ingest_service
            .ingest(&event, &raw_payload)
            .await
            .map_err(classify_ingest_error)
    }

    async fn publish_to_dlq(&self, channel: &Channel, payload: &[u8]) -> AppResult<()> {
        channel
            .basic_publish(
                self.dlx_name(),
                self.dlq_routing_key(),
                BasicPublishOptions::default(),
                payload,
                BasicProperties::default(),
            )
            .await?
            .await?;

        Ok(())
    }

    fn queue_name(&self) -> ShortString {
        ShortString::from(self.config.rabbitmq_queue.as_str())
    }

    fn exchange_name(&self) -> ShortString {
        ShortString::from(self.config.rabbitmq_exchange.as_str())
    }

    fn dlx_name(&self) -> ShortString {
        ShortString::from(self.config.rabbitmq_dlx.as_str())
    }

    fn dlq_name(&self) -> ShortString {
        ShortString::from(self.config.rabbitmq_dlq.as_str())
    }

    fn routing_key(&self) -> ShortString {
        ShortString::from(self.config.rabbitmq_routing_key.as_str())
    }

    fn dlq_routing_key(&self) -> ShortString {
        ShortString::from(self.config.rabbitmq_dlq_routing_key.as_str())
    }

    fn consumer_tag(&self) -> ShortString {
        ShortString::from(self.config.consumer_tag.as_str())
    }
}

enum MessageProcessingError {
    InvalidMessage(String),
    TemporaryFailure(AppError),
}

fn classify_ingest_error(error: AppError) -> MessageProcessingError {
    match error {
        AppError::Validation(message) => MessageProcessingError::InvalidMessage(message),
        other => MessageProcessingError::TemporaryFailure(other),
    }
}