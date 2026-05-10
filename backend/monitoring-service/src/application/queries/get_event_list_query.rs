use std::sync::Arc;

use time::OffsetDateTime;

use crate::{
    application::ports::events_repository::{
        EventListItemView,
        EventListQuery,
        EventsRepository,
    },
    shared::error::{AppError, AppResult},
};

#[derive(Clone)]
pub struct GetEventListQuery {
    repository: Arc<dyn EventsRepository>,
}

#[derive(Debug, Clone)]
pub struct GetEventListInput {
    pub service: Option<String>,
    pub event_type: Option<String>,
    pub trace_id: Option<String>,
    pub idempotency_key: Option<String>,
    pub operation: Option<String>,
    pub transport: Option<String>,
    pub from: Option<OffsetDateTime>,
    pub to: Option<OffsetDateTime>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl GetEventListQuery {
    pub fn new(repository: Arc<dyn EventsRepository>) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: GetEventListInput,
    ) -> AppResult<Vec<EventListItemView>> {
        let limit = input.limit.unwrap_or(50);
        let offset = input.offset.unwrap_or(0);

        if limit <= 0 {
            return Err(AppError::validation("limit must be > 0"));
        }

        if limit > 200 {
            return Err(AppError::validation("limit must be <= 200"));
        }

        if offset < 0 {
            return Err(AppError::validation("offset must be >= 0"));
        }

        if let (Some(from), Some(to)) = (input.from, input.to) {
            if from > to {
                return Err(AppError::validation("from must be <= to"));
            }
        }

        self.repository
            .get_event_list(EventListQuery {
                service: normalize(input.service),
                event_type: normalize(input.event_type),
                trace_id: normalize(input.trace_id),
                idempotency_key: normalize(input.idempotency_key),
                operation: normalize(input.operation),
                transport: normalize_transport(input.transport)?,
                from: input.from,
                to: input.to,
                limit,
                offset,
            })
            .await
    }
}

fn normalize(value: Option<String>) -> Option<String> {
    value.and_then(|x| {
        let trimmed = x.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed)
        }
    })
}

fn normalize_transport(value: Option<String>) -> AppResult<Option<String>> {
    let Some(value) = value else {
        return Ok(None);
    };

    let normalized = value.trim().to_uppercase();

    if normalized.is_empty() {
        return Ok(None);
    }

    if normalized != "HTTP" && normalized != "WS" {
        return Err(AppError::validation("transport must be HTTP or WS"));
    }

    Ok(Some(normalized))
}