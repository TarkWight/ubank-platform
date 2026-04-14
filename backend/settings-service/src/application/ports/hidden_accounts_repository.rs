use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    domain::settings::enums::AppKind,
    infrastructure::persistence::error::RepositoryError,
};

#[async_trait]
pub trait HiddenAccountsRepository: Send + Sync {
    async fn list(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
    ) -> Result<Vec<String>, RepositoryError>;

    async fn add(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        account_id: &str,
    ) -> Result<(), RepositoryError>;

    async fn remove(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        account_id: &str,
    ) -> Result<(), RepositoryError>;
}