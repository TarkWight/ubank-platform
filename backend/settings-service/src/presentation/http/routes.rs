use axum::{
    middleware,
    routing::{delete, get, put},
    Extension, Router,
};

use crate::{
    infrastructure::auth::middleware::{auth_middleware, AuthState},
    presentation::http::handlers::{
        get_settings, health, hide_account, list_hidden_accounts, unhide_account, update_locale,
        update_theme, HttpState,
    },
};

pub fn build_router(http_state: HttpState, auth_state: AuthState) -> Router {
    let protected = Router::new()
        .route(
            "/api/settings/{app_kind}",
            get(get_settings)
        )
        .route(
            "/api/settings/{app_kind}/theme",
            put(update_theme)
        )
        .route(
            "/api/settings/{app_kind}/locale",
            put(update_locale)
        )
        .route(
            "/api/settings/{app_kind}/hidden-accounts",
            get(list_hidden_accounts),
        )
        .route(
            "/api/settings/{app_kind}/hidden-accounts/{account_id}",
            put(hide_account).delete(unhide_account),
        )
        .layer(middleware::from_fn(auth_middleware))
        .layer(Extension(auth_state));

    Router::new()
        .route("/health", get(health))
        .merge(protected)
        .with_state(http_state)
}