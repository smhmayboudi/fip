use crate::proto::{AtRes, AtSaveReq, AtUpdateReq};
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
    pub header_typ: String,
    /// TODO: documentation
    pub header_alg: String,
    /// TODO: documentation
    pub header_cty: String,
    /// TODO: documentation
    pub header_jku: String,
    /// TODO: documentation
    pub header_kid: String,
    /// TODO: documentation
    pub header_x5u: String,
    /// TODO: documentation
    pub header_x5t: String,
    /// TODO: documentation
    pub token_blocked: bool,
    /// TODO: documentation
    pub token_blocked_description: String,
}

/// TODO: documentation
impl From<&AtSaveReq> for Model {
    /// TODO: documentation
    fn from(req: &AtSaveReq) -> Self {
        Self {
            claims_aud: req.claims_aud.clone(),
            claims_exp: req.claims_exp,
            claims_iat: req.claims_iat,
            claims_iss: req.claims_iss.clone(),
            claims_jti: req.claims_jti.clone(),
            claims_nbf: req.claims_nbf,
            claims_sub: req.claims_sub.clone(),
            header_typ: req.header_typ.clone(),
            header_alg: req.header_alg.clone(),
            header_cty: req.header_cty.clone(),
            header_jku: req.header_jku.clone(),
            header_kid: req.header_kid.clone(),
            header_x5u: req.header_x5u.clone(),
            header_x5t: req.header_x5t.clone(),
            token_blocked: req.token_blocked,
            token_blocked_description: req.token_blocked_description.clone(),
        }
    }
}

/// TODO: documentation
impl From<&AtUpdateReq> for Model {
    /// TODO: documentation
    fn from(req: &AtUpdateReq) -> Self {
        Self {
            claims_aud: req.claims_aud.clone(),
            claims_exp: req.claims_exp,
            claims_iat: req.claims_iat,
            claims_iss: req.claims_iss.clone(),
            claims_jti: req.claims_jti.clone(),
            claims_nbf: req.claims_nbf,
            claims_sub: req.claims_sub.clone(),
            header_typ: req.header_typ.clone(),
            header_alg: req.header_alg.clone(),
            header_cty: req.header_cty.clone(),
            header_jku: req.header_jku.clone(),
            header_kid: req.header_kid.clone(),
            header_x5u: req.header_x5u.clone(),
            header_x5t: req.header_x5t.clone(),
            token_blocked: req.token_blocked,
            token_blocked_description: req.token_blocked_description.clone(),
        }
    }
}

/// TODO: documentation
impl From<Model> for AtRes {
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
            header_typ: req.header_typ.clone(),
            header_alg: req.header_alg.clone(),
            header_cty: req.header_cty.clone(),
            header_jku: req.header_jku.clone(),
            header_kid: req.header_kid.clone(),
            header_x5u: req.header_x5u.clone(),
            header_x5t: req.header_x5t.clone(),
            token_blocked: req.token_blocked,
            token_blocked_description: req.token_blocked_description,
        }
    }
}
