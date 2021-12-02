use crate::{
    config::Config,
    model::Model,
    proto::{
        JwksDeleteReq, JwksFindOneRandomReq, JwksFindOneReq, JwksFindReq, JwksRes, JwksSaveReq,
        JwksUpdateReq,
    },
    repository::Repository,
};
use fip_common::error::Error;
use uuid::Uuid;

/// TODO: documentation
#[allow(dead_code)]
#[derive(Debug)]
pub struct Service {
    config: Config,
    repository: Repository,
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[must_use]
    pub const fn new(config: Config, repository: Repository) -> Self {
        Self { config, repository }
    }
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete(&self, req: &JwksDeleteReq) -> Result<JwksRes, Error> {
        let res = self.repository.find_one(&req.id).await?;
        let _rows_affected = self.repository.delete(&req.id).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find(&self, req: &JwksFindReq) -> Result<Vec<JwksRes>, Error> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(Into::into).collect())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one(&self, req: &JwksFindOneReq) -> Result<JwksRes, Error> {
        let res = self.repository.find_one(&req.id).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one_random(&self, req: &JwksFindOneRandomReq) -> Result<JwksRes, Error> {
        let res = self.repository.find_one_random().await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn save(&self, req: &JwksSaveReq) -> Result<JwksRes, Error> {
        let mut model: Model = req.into();
        model.id = Uuid::new_v4().to_string().to_uppercase();
        let _rows_affected = self.repository.save(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn update(&self, req: &JwksUpdateReq) -> Result<JwksRes, Error> {
        let model = req.into();
        let _rows_affected = self.repository.update(&model).await?;
        Ok(model.into())
    }
}
