use axum::{
    extract::Request,
    http::header::AUTHORIZATION,
    middleware::Next,
    response::Response,
};
use tracing::{info, warn};

use crate::{
    infrastructure::auth::{
        auth_context::AuthContext,
        error::AuthError,
        jwt_validator::JwtValidator,
    },
    presentation::http::error_response::HttpError,
};

#[derive(Clone)]
pub struct AuthState {
    pub jwt_validator: JwtValidator,
}

pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, HttpError> {
    info!("auth middleware: started");

    let auth_state = request
        .extensions()
        .get::<AuthState>()
        .cloned()
        .ok_or_else(|| {
            warn!("auth middleware: AuthState missing in request extensions");
            HttpError::Internal
        })?;

    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or_else(|| {
            warn!("auth middleware: Authorization header is missing");
            AuthError::MissingAuthorizationHeader
        })?
        .to_str()
        .map_err(|_| {
            warn!("auth middleware: Authorization header is not valid UTF-8");
            AuthError::InvalidAuthorizationHeader
        })?;

    info!("auth middleware: Authorization header received");

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or_else(|| {
            warn!("auth middleware: Authorization header does not start with 'Bearer '");
            AuthError::InvalidAuthorizationHeader
        })?;

    info!("auth middleware: Bearer token extracted");

    let auth_context: AuthContext = auth_state.jwt_validator.validate(token).map_err(|err| {
        warn!("auth middleware: JWT validation failed: {:?}", err);
        err
    })?;

    info!(
        "auth middleware: JWT validated successfully for user_id={}",
        auth_context.user_id
    );

    request.extensions_mut().insert(auth_context);

    Ok(next.run(request).await)
}