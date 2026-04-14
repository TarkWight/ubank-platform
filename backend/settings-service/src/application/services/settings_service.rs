use std::sync::Arc;
use uuid::Uuid;

use crate::{
    application::{
        dto::responses::{HiddenAccountsResponse, SettingsResponse},
        error::ApplicationError,
        ports::{
            hidden_accounts_repository::HiddenAccountsRepository,
            settings_repository::SettingsRepository,
        },
    },
    domain::settings::enums::{AppKind, Theme},
};

#[derive(Clone)]
pub struct SettingsApplicationService {
    settings_repo: Arc<dyn SettingsRepository>,
    hidden_repo: Arc<dyn HiddenAccountsRepository>,
    default_locale: String,
}

impl SettingsApplicationService {
    pub fn new(
        settings_repo: Arc<dyn SettingsRepository>,
        hidden_repo: Arc<dyn HiddenAccountsRepository>,
        default_locale: String,
    ) -> Self {
        Self {
            settings_repo,
            hidden_repo,
            default_locale,
        }
    }

    pub async fn get_settings(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
    ) -> Result<SettingsResponse, ApplicationError> {
        let settings = self
            .settings_repo
            .find_by_user_and_app(user_id, app_kind)
            .await?;

        let hidden_account_ids = self.hidden_repo.list(user_id, app_kind).await?;

        match settings {
            Some(settings) => Ok(SettingsResponse {
                app_kind,
                theme: settings.theme,
                locale: settings.locale,
                hidden_account_ids,
            }),
            None => Ok(SettingsResponse {
                app_kind,
                theme: Theme::System,
                locale: self.default_locale.clone(),
                hidden_account_ids,
            }),
        }
    }

    pub async fn update_theme(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        theme: Theme,
    ) -> Result<SettingsResponse, ApplicationError> {
        let existing = self
            .settings_repo
            .find_by_user_and_app(user_id, app_kind)
            .await?;

        let locale = existing
            .as_ref()
            .map(|s| s.locale.clone())
            .unwrap_or_else(|| self.default_locale.clone());

        let updated = self
            .settings_repo
            .upsert(user_id, app_kind, theme, &locale)
            .await?;

        let hidden_account_ids = self.hidden_repo.list(user_id, app_kind).await?;

        Ok(SettingsResponse {
            app_kind,
            theme: updated.theme,
            locale: updated.locale,
            hidden_account_ids,
        })
    }

    pub async fn update_locale(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        locale: String,
    ) -> Result<SettingsResponse, ApplicationError> {
        validate_locale(&locale)?;

        let existing = self
            .settings_repo
            .find_by_user_and_app(user_id, app_kind)
            .await?;

        let theme = existing
            .as_ref()
            .map(|s| s.theme)
            .unwrap_or(Theme::System);

        let updated = self
            .settings_repo
            .upsert(user_id, app_kind, theme, &locale)
            .await?;

        let hidden_account_ids = self.hidden_repo.list(user_id, app_kind).await?;

        Ok(SettingsResponse {
            app_kind,
            theme: updated.theme,
            locale: updated.locale,
            hidden_account_ids,
        })
    }

    pub async fn list_hidden_accounts(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
    ) -> Result<HiddenAccountsResponse, ApplicationError> {
        let account_ids = self.hidden_repo.list(user_id, app_kind).await?;
        Ok(HiddenAccountsResponse { account_ids })
    }

    pub async fn hide_account(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        account_id: String,
    ) -> Result<(), ApplicationError> {
        validate_account_id(&account_id)?;
        self.hidden_repo.add(user_id, app_kind, &account_id).await?;
        Ok(())
    }

    pub async fn unhide_account(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        account_id: String,
    ) -> Result<(), ApplicationError> {
        validate_account_id(&account_id)?;
        self.hidden_repo.remove(user_id, app_kind, &account_id).await?;
        Ok(())
    }
}

fn validate_locale(locale: &str) -> Result<(), ApplicationError> {
    if locale.trim().is_empty() || locale.len() > 16 {
        return Err(ApplicationError::InvalidLocale);
    }
    Ok(())
}

fn validate_account_id(account_id: &str) -> Result<(), ApplicationError> {
    if account_id.trim().is_empty() || account_id.len() > 128 {
        return Err(ApplicationError::InvalidAccountId);
    }
    Ok(())
}