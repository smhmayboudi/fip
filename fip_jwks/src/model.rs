use crate::proto::{JwksRes, JwksSaveReq, JwksUpdateReq};
use sqlx::FromRow;

/// TODO: documentation
#[derive(Debug, FromRow)]
pub struct Model {
    /// TODO: documentation
    pub id: String,
    /// TODO: documentation
    pub private_key: String,
    /// TODO: documentation
    pub public_key: String,
}

/// TODO: documentation
impl From<&JwksSaveReq> for Model {
    /// TODO: documentation
    fn from(req: &JwksSaveReq) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key.clone(),
        }
    }
}

/// TODO: documentation
impl From<&JwksUpdateReq> for Model {
    /// TODO: documentation
    fn from(req: &JwksUpdateReq) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key.clone(),
        }
    }
}

/// TODO: documentation
impl From<Model> for JwksRes {
    /// TODO: documentation
    fn from(req: Model) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key,
        }
    }
}
