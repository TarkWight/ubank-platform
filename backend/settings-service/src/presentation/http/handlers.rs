use std::sync::Arc;

use axum::{
    extract::{Extension, Path, State},
    Json,
};

use crate::{
    application::{
        dto::{
            requests::{UpdateLocaleRequest, UpdateThemeRequest},
            responses::{HiddenAccountsResponse, SettingsResponse},
        },
        error::ApplicationError,
        services::settings_service::SettingsApplicationService,
    },
    domain::settings::enums::{AppKind, Role},
    infrastructure::auth::auth_context::AuthContext,
    presentation::http::error_response::HttpError,
};

#[derive(Clone)]
pub struct HttpState {
    pub settings_service: Arc<SettingsApplicationService>,
}

fn ensure_role_for_app_kind(
    auth: &AuthContext,
    app_kind: AppKind,
) -> Result<(), ApplicationError> {
    let allowed = match app_kind {
        AppKind::Client => auth.roles.contains(&Role::Client),
        AppKind::Employee => auth.roles.contains(&Role::Employee),
    };

    if allowed {
        Ok(())
    } else {
        Err(ApplicationError::Forbidden)
    }
}

pub async fn health() -> &'static str {
    "ok"
}

pub async fn get_settings(
    State(state): State<HttpState>,
    Extension(auth): Extension<AuthContext>,
    Path(app_kind): Path<AppKind>,
) -> Result<Json<SettingsResponse>, HttpError> {
    ensure_role_for_app_kind(&auth, app_kind)?;

    let response = state
        .settings_service
        .get_settings(auth.user_id, app_kind)
        .await
        .map_err(|err| {
            tracing::error!("get_settings failed: {:?}", err);
            err
        })?;

    Ok(Json(response))
}
pub async fn update_theme(
    State(state): State<HttpState>,
    Extension(auth): Extension<AuthContext>,
    Path(app_kind): Path<AppKind>,
    Json(request): Json<UpdateThemeRequest>,
) -> Result<Json<SettingsResponse>, HttpError> {
    ensure_role_for_app_kind(&auth, app_kind)?;

    let response = state
        .settings_service
        .update_theme(auth.user_id, app_kind, request.theme)
        .await?;

    Ok(Json(response))
}

pub async fn update_locale(
    State(state): State<HttpState>,
    Extension(auth): Extension<AuthContext>,
    Path(app_kind): Path<AppKind>,
    Json(request): Json<UpdateLocaleRequest>,
) -> Result<Json<SettingsResponse>, HttpError> {
    ensure_role_for_app_kind(&auth, app_kind)?;

    let response = state
        .settings_service
        .update_locale(auth.user_id, app_kind, request.locale)
        .await?;

    Ok(Json(response))
}

pub async fn list_hidden_accounts(
    State(state): State<HttpState>,
    Extension(auth): Extension<AuthContext>,
    Path(app_kind): Path<AppKind>,
) -> Result<Json<HiddenAccountsResponse>, HttpError> {
    ensure_role_for_app_kind(&auth, app_kind)?;

    let response = state
        .settings_service
        .list_hidden_accounts(auth.user_id, app_kind)
        .await?;

    Ok(Json(response))
}

pub async fn hide_account(
    State(state): State<HttpState>,
    Extension(auth): Extension<AuthContext>,
    Path((app_kind, account_id)): Path<(AppKind, String)>,
) -> Result<(), HttpError> {
    ensure_role_for_app_kind(&auth, app_kind)?;

    state
        .settings_service
        .hide_account(auth.user_id, app_kind, account_id)
        .await?;

    Ok(())
}

pub async fn unhide_account(
    State(state): State<HttpState>,
    Extension(auth): Extension<AuthContext>,
    Path((app_kind, account_id)): Path<(AppKind, String)>,
) -> Result<(), HttpError> {
    ensure_role_for_app_kind(&auth, app_kind)?;

    state
        .settings_service
        .unhide_account(auth.user_id, app_kind, account_id)
        .await?;

    Ok(())
}