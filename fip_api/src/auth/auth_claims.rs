use serde::{Deserialize, Serialize};

/// TODO: documentation
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthClaims {
    /// Optional. Audience
    pub aud: String,
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub exp: i64,
    /// Optional. Issued at (as UTC timestamp)
    pub iat: i64,
    /// Optional. Issuer
    pub iss: String,
    /// Optional. jwt id
    pub jti: String,
    /// Optional. Not Before (as UTC timestamp)
    pub nbf: i64,
    /// Optional. Subject (whom token refers to)
    pub sub: String,
}
