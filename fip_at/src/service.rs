use crate::{
    config::Config,
    model::Model,
    proto::{
        AtDeleteByClaimsSubReq, AtDeleteReq, AtFindOneByClaimsSubReq, AtFindOneReq, AtFindReq,
        AtRes, AtSaveReq, AtUpdateReq, AtValidateByClaimsSubReq, AtValidateReq,
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
    pub async fn delete(&self, req: &AtDeleteReq) -> Result<AtRes, Error> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        let _rows_affected = self.repository.delete(&req.claims_jti).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete_by_claims_sub(&self, req: &AtDeleteByClaimsSubReq) -> Result<AtRes, Error> {
        let res = self
            .repository
            .find_one_by_claims_sub(&req.claims_sub)
            .await?;
        let _rows_affected = self
            .repository
            .delete_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find(&self, req: &AtFindReq) -> Result<Vec<AtRes>, Error> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(Into::into).collect())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one(&self, req: &AtFindOneReq) -> Result<AtRes, Error> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one_by_claims_sub(
        &self,
        req: &AtFindOneByClaimsSubReq,
    ) -> Result<AtRes, Error> {
        let res = self
            .repository
            .find_one_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn save(&self, req: &AtSaveReq) -> Result<AtRes, Error> {
        let mut model: Model = req.into();
        model.claims_jti = Uuid::new_v4().to_string().to_uppercase();
        let _rows_affected = self.repository.save(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn update(&self, req: &AtUpdateReq) -> Result<AtRes, Error> {
        let model = req.into();
        let _rows_affected = self.repository.update(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn validate(&self, req: &AtValidateReq) -> Result<AtRes, Error> {
        let res = self.repository.validate(&req.claims_jti).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn validate_by_claims_sub(
        &self,
        req: &AtValidateByClaimsSubReq,
    ) -> Result<AtRes, Error> {
        let res = self
            .repository
            .validate_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }
}
