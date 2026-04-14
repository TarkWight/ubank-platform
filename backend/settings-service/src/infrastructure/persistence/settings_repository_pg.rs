use async_trait::async_trait;
use sqlx::{PgPool, Row};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{
    application::ports::settings_repository::SettingsRepository,
    domain::settings::{
        enums::{AppKind, Theme},
        model::UserSettings,
    },
    infrastructure::persistence::error::RepositoryError,
};

pub struct SettingsRepositoryPg {
    pool: PgPool,
}

impl SettingsRepositoryPg {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SettingsRepository for SettingsRepositoryPg {
    async fn find_by_user_and_app(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
    ) -> Result<Option<UserSettings>, RepositoryError> {
        let row = sqlx::query(
            r#"
            select user_id, app_kind, theme, locale, version, created_at, updated_at
            from user_settings
            where user_id = $1 and app_kind = $2
            "#,
        )
            .bind(user_id)
            .bind(app_kind.as_db_str())
            .fetch_optional(&self.pool)
            .await?;

        let Some(row) = row else {
            return Ok(None);
        };

        let app_kind_raw: String = row.try_get("app_kind")?;
        let theme_raw: String = row.try_get("theme")?;

        let app_kind = app_kind_raw
            .parse()
            .map_err(|_| RepositoryError::Database(sqlx::Error::Protocol("invalid app_kind in db".into())))?;

        let theme = theme_raw
            .parse()
            .map_err(|_| RepositoryError::Database(sqlx::Error::Protocol("invalid theme in db".into())))?;

        Ok(Some(UserSettings {
            user_id: row.try_get("user_id")?,
            app_kind,
            theme,
            locale: row.try_get("locale")?,
            version: row.try_get("version")?,
            created_at: row.try_get::<OffsetDateTime, _>("created_at")?,
            updated_at: row.try_get::<OffsetDateTime, _>("updated_at")?,
        }))
    }

    async fn upsert(
        &self,
        user_id: Uuid,
        app_kind: AppKind,
        theme: Theme,
        locale: &str,
    ) -> Result<UserSettings, RepositoryError> {
        let now = OffsetDateTime::now_utc();

        let row = sqlx::query(
            r#"
            insert into user_settings (
                user_id, app_kind, theme, locale, version, created_at, updated_at
            )
            values ($1, $2, $3, $4, 0, $5, $5)
            on conflict (user_id, app_kind)
            do update set
                theme = excluded.theme,
                locale = excluded.locale,
                version = user_settings.version + 1,
                updated_at = excluded.updated_at
            returning user_id, app_kind, theme, locale, version, created_at, updated_at
            "#,
        )
            .bind(user_id)
            .bind(app_kind.as_db_str())
            .bind(theme.as_db_str())
            .bind(locale)
            .bind(now)
            .fetch_one(&self.pool)
            .await?;

        let app_kind_raw: String = row.try_get("app_kind")?;
        let theme_raw: String = row.try_get("theme")?;

        let app_kind = app_kind_raw
            .parse()
            .map_err(|_| RepositoryError::Database(sqlx::Error::Protocol("invalid app_kind in db".into())))?;

        let theme = theme_raw
            .parse()
            .map_err(|_| RepositoryError::Database(sqlx::Error::Protocol("invalid theme in db".into())))?;

        Ok(UserSettings {
            user_id: row.try_get("user_id")?,
            app_kind,
            theme,
            locale: row.try_get("locale")?,
            version: row.try_get("version")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}