use crate::{
    config::Config,
    model::Model,
    proto::{
        RtDeleteByClaimsSubReq, RtDeleteReq, RtFindOneByClaimsSubReq, RtFindOneReq, RtFindReq,
        RtRes, RtSaveReq, RtUpdateReq, RtValidateByClaimsSubReq, RtValidateReq,
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
    pub async fn delete(&self, req: &RtDeleteReq) -> Result<RtRes, Error> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        let _e = self.repository.delete(&req.claims_jti).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete_by_claims_sub(&self, req: &RtDeleteByClaimsSubReq) -> Result<RtRes, Error> {
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
    pub async fn find(&self, req: &RtFindReq) -> Result<Vec<RtRes>, Error> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(Into::into).collect())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one(&self, req: &RtFindOneReq) -> Result<RtRes, Error> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one_by_claims_sub(
        &self,
        req: &RtFindOneByClaimsSubReq,
    ) -> Result<RtRes, Error> {
        let res = self
            .repository
            .find_one_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn save(&self, req: &RtSaveReq) -> Result<RtRes, Error> {
        let mut model: Model = req.into();
        model.claims_jti = Uuid::new_v4().to_string().to_uppercase();
        let _rows_affected = self.repository.save(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn update(&self, req: &RtUpdateReq) -> Result<RtRes, Error> {
        let model = req.into();
        let _rows_affected = self.repository.update(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn validate(&self, req: &RtValidateReq) -> Result<RtRes, Error> {
        let res = self.repository.validate(&req.claims_jti).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn validate_by_claims_sub(
        &self,
        req: &RtValidateByClaimsSubReq,
    ) -> Result<RtRes, Error> {
        let res = self
            .repository
            .validate_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }
}
