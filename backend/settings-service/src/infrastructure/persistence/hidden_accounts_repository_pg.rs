use async_trait::async_trait;
use sqlx::{PgPool, Row};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    application::ports::hidden_accounts_repository::HiddenAccountsRepository,
    domain::settings::enums::AppKind,
    infrastructure::persistence::error::RepositoryError,
};

pub struct HiddenAccountsRepositoryPg {
    pool: PgPool,
}

impl HiddenAccountsRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl HiddenAccountsRepository for HiddenAccountsRepositoryPg {
    async fn list(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
    ) -> Result<Vec<String>, RepositoryError> {
        let rows = sqlx::query(
            r#"
            select account_id
            from hidden_accounts
            where user_id = $1 and app_kind = $2
            order by created_at asc
            "#,
        )
            .bind(user_id)
            .bind(app_kind.as_db_str())
            .fetch_all(&self.pool)
            .await?;

        let values = rows
            .into_iter()
            .map(|row| row.try_get::<String, _>("account_id"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(values)
    }

    async fn add(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        account_id: &str,
    ) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            insert into hidden_accounts (user_id, app_kind, account_id, created_at)
            values ($1, $2, $3, $4)
            on conflict (user_id, app_kind, account_id) do nothing
            "#,
        )
            .bind(user_id)
            .bind(app_kind.as_db_str())
            .bind(account_id)
            .bind(OffsetDateTime::now_utc())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn remove(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        account_id: &str,
    ) -> Result<(), RepositoryError> {
        sqlx::query(
            r#"
            delete from hidden_accounts
            where user_id = $1 and app_kind = $2 and account_id = $3
            "#,
        )
            .bind(user_id)
            .bind(app_kind.as_db_str())
            .bind(account_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}