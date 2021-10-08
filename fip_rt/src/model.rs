use crate::proto::{RtRes, RtSaveReq, RtUpdateReq};
use sqlx::FromRow;

/// TODO: documentation
#[derive(Debug, FromRow)]
pub struct Model {
    /// TODO: documentation
    pub claims_aud: String,
    /// TODO: documentation
    pub claims_exp: i64,
    /// TODO: documentation
    pub claims_iat: i64,
    /// TODO: documentation
    pub claims_iss: String,
    /// TODO: documentation
    pub claims_jti: String,
    /// TODO: documentation
    pub claims_nbf: i64,
    /// TODO: documentation
    pub claims_sub: String,
    /// TODO: documentation
    pub token_blocked: bool,
    /// TODO: documentation
    pub token_blocked_description: String,
}

/// TODO: documentation
impl From<&RtSaveReq> for Model {
    /// TODO: documentation
    fn from(req: &RtSaveReq) -> Self {
        Self {
            claims_aud: req.claims_aud.clone(),
            claims_exp: req.claims_exp,
            claims_iat: req.claims_iat,
            claims_iss: req.claims_iss.clone(),
            claims_jti: req.claims_jti.clone(),
            claims_nbf: req.claims_nbf,
            claims_sub: req.claims_sub.clone(),
            token_blocked: req.token_blocked,
            token_blocked_description: req.token_blocked_description.clone(),
        }
    }
}

/// TODO: documentation
impl From<&RtUpdateReq> for Model {
    /// TODO: documentation
    fn from(req: &RtUpdateReq) -> Self {
        Self {
            claims_aud: req.claims_aud.clone(),
            claims_exp: req.claims_exp,
            claims_iat: req.claims_iat,
            claims_iss: req.claims_iss.clone(),
            claims_jti: req.claims_jti.clone(),
            claims_nbf: req.claims_nbf,
            claims_sub: req.claims_sub.clone(),
            token_blocked: req.token_blocked,
            token_blocked_description: req.token_blocked_description.clone(),
        }
    }
}

/// TODO: documentation
impl From<Model> for RtRes {
    /// TODO: documentation
    fn from(req: Model) -> Self {
        Self {
            claims_aud: req.claims_aud.clone(),
            claims_exp: req.claims_exp,
            claims_iat: req.claims_iat,
            claims_iss: req.claims_iss.clone(),
            claims_jti: req.claims_jti.clone(),
            claims_nbf: req.claims_nbf,
            claims_sub: req.claims_sub.clone(),
            token_blocked: req.token_blocked,
            token_blocked_description: req.token_blocked_description,
        }
    }
}
