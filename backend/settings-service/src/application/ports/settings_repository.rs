use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::settings::{
        enums::{AppKind, Theme},
        model::UserSettings,
    },
    infrastructure::persistence::error::RepositoryError,
};

#[async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn find_by_user_and_app(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
    ) -> Result<Option<UserSettings>, RepositoryError>;

    async fn upsert(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        theme: Theme,
        locale: &str,
    ) -> Result<UserSettings, RepositoryError>;
}