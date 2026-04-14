use crate::domain::settings::enums::{AppKind, Theme};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SettingsResponse {
    pub app_kind: AppKind,
    pub theme: Theme,
    pub locale: String,
    pub hidden_account_ids: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct HiddenAccountsResponse {
    pub account_ids: Vec<String>,
}