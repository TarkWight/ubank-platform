use crate::domain::settings::enums::Theme;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UpdateThemeRequest {
    pub theme: Theme,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLocaleRequest {
    pub locale: String,
}