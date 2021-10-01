use crate::proto::{JwksRes, JwksSaveReq, JwksUpdateReq};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Model {
    pub id: String,
    pub private_key: String,
    pub public_key: String,
}

impl From<&JwksSaveReq> for Model {
    fn from(req: &JwksSaveReq) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key.clone(),
        }
    }
}

impl From<&JwksUpdateReq> for Model {
    fn from(req: &JwksUpdateReq) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key.clone(),
        }
    }
}

impl Into<JwksRes> for Model {
    fn into(self) -> JwksRes {
        JwksRes {
            id: self.id,
            private_key: self.private_key,
            public_key: self.public_key,
        }
    }
}
