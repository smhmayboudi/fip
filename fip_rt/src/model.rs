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
impl Into<RtRes> for Model {
    /// TODO: documentation
    fn into(self) -> RtRes {
        RtRes {
            claims_aud: self.claims_aud,
            claims_exp: self.claims_exp,
            claims_iat: self.claims_iat,
            claims_iss: self.claims_iss,
            claims_jti: self.claims_jti,
            claims_nbf: self.claims_nbf,
            claims_sub: self.claims_sub,
            token_blocked: self.token_blocked,
            token_blocked_description: self.token_blocked_description,
        }
    }
}
