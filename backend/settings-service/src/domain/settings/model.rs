use super::enums::{AppKind, Theme};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserSettings {
    pub user_id: Uuid,
    pub app_kind: AppKind,
    pub theme: Theme,
    pub locale: String,
    pub version: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

#[derive(Debug, Clone)]
pub struct HiddenAccount {
    pub user_id: Uuid,
    pub app_kind: AppKind,
    pub account_id: String,
    pub created_at: OffsetDateTime,
}