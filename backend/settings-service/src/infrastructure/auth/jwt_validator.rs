use jsonwebtoken::{
    decode,
    errors::ErrorKind,
    Algorithm,
    DecodingKey,
    Validation,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    domain::settings::enums::Role,
    infrastructure::auth::{
        auth_context::AuthContext,
        claims::JwtClaims,
        error::{AuthError, AuthInitError},
    },
};

#[derive(Clone)]
pub struct JwtValidator {
    decoding_key: DecodingKey,
    validation: Validation,
}

impl JwtValidator {
    pub fn new(secret: &str) -> Result<Self, AuthInitError> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;

        let decoding_key = DecodingKey::from_base64_secret(secret)
            .map_err(|_| AuthInitError::InvalidJwtSecret)?;

        Ok(Self {
            decoding_key,
            validation,
        })
    }

    pub fn validate(&self, token: &str) -> Result<AuthContext, AuthError> {
        info!("jwt validator: starting decode");

        let token_data = decode::<JwtClaims>(token, &self.decoding_key, &self.validation)
            .map_err(|err| {
                warn!("jwt validator: decode failed: {:?}", err.kind());
                match err.kind() {
                    ErrorKind::ExpiredSignature => AuthError::ExpiredToken,
                    _ => AuthError::InvalidToken,
                }
            })?;

        info!("jwt validator: token decoded");

        let user_id = Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| {
                warn!(
                    "jwt validator: sub is not a valid UUID: {}",
                    token_data.claims.sub
                );
                AuthError::InvalidSubject
            })?;

        let roles = token_data
            .claims
            .roles
            .iter()
            .map(|role| {
                Role::from_claim(role).map_err(|_| {
                    warn!("jwt validator: unsupported role claim: {}", role);
                    AuthError::UnsupportedRoleClaim
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        info!("jwt validator: roles parsed successfully");

        Ok(AuthContext { user_id, roles })
    }
}