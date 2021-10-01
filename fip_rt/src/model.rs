use crate::proto::{RtRes, RtSaveReq, RtUpdateReq};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Model {
    pub claims_aud: String,
    pub claims_exp: i64,
    pub claims_iat: i64,
    pub claims_iss: String,
    pub claims_jti: String,
    pub claims_nbf: i64,
    pub claims_sub: String,
    pub token_blocked: bool,
    pub token_blocked_description: String,
}

impl From<&RtSaveReq> for Model {
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

impl From<&RtUpdateReq> for Model {
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

impl Into<RtRes> for Model {
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
