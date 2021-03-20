use crate::proto::{JwksResDto, JwksSaveReqDto, JwksUpdateReqDto};
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Model {
    pub id: String,
    pub private_key: String,
    pub public_key: String,
}

impl From<&JwksSaveReqDto> for Model {
    fn from(req: &JwksSaveReqDto) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key.clone(),
        }
    }
}

impl From<&JwksUpdateReqDto> for Model {
    fn from(req: &JwksUpdateReqDto) -> Self {
        Self {
            id: req.id.clone(),
            private_key: req.private_key.clone(),
            public_key: req.public_key.clone(),
        }
    }
}

impl Into<JwksResDto> for Model {
    fn into(self) -> JwksResDto {
        JwksResDto {
            id: self.id,
            private_key: self.private_key,
            public_key: self.public_key,
        }
    }
}
