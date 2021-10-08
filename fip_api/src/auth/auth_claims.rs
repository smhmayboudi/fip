use serde::{Deserialize, Serialize};

/// TODO: documentation
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthClaims {
    /// Optional. Audience
    pub aud: String,
    /// Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub exp: usize,
    /// Optional. Issued at (as UTC timestamp)
    pub iat: usize,
    /// Optional. Issuer
    pub iss: String,
    /// Optional. jwt id
    pub jti: String,

    /// Optional. Not Before (as UTC timestamp)
    pub nbf: usize,

    /// Optional. Subject (whom token refers to)
    pub sub: String,
}
