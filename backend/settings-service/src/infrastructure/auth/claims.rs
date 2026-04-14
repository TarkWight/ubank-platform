use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: usize,
    pub iat: usize,
}