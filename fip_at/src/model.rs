use crate::proto::{AtResDto, AtSaveReqDto, AtUpdateReqDto};
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
    pub header_typ: String,
    pub header_alg: String,
    pub header_cty: String,
    pub header_jku: String,
    pub header_kid: String,
    pub header_x5u: String,
    pub header_x5t: String,
    pub token_blocked: bool,
    pub token_blocked_description: String,
}

impl From<&AtSaveReqDto> for Model {
    fn from(req: &AtSaveReqDto) -> Self {
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

impl From<&AtUpdateReqDto> for Model {
    fn from(req: &AtUpdateReqDto) -> Self {
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

impl Into<AtResDto> for Model {
    fn into(self) -> AtResDto {
        AtResDto {
            claims_aud: self.claims_aud,
            claims_exp: self.claims_exp,
            claims_iat: self.claims_iat,
            claims_iss: self.claims_iss,
            claims_jti: self.claims_jti,
            claims_nbf: self.claims_nbf,
            claims_sub: self.claims_sub,
            header_typ: self.header_typ,
            header_alg: self.header_alg,
            header_cty: self.header_cty,
            header_jku: self.header_jku,
            header_kid: self.header_kid,
            header_x5u: self.header_x5u,
            header_x5t: self.header_x5t,
            token_blocked: self.token_blocked,
            token_blocked_description: self.token_blocked_description,
        }
    }
}
