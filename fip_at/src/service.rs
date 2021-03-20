use crate::{
    config::Config,
    model::Model,
    proto::{
        AtDeleteByClaimsSubReqDto, AtDeleteReqDto, AtFindOneByClaimsSubReqDto, AtFindOneReqDto,
        AtFindReqDto, AtResDto, AtSaveReqDto, AtUpdateReqDto, AtValidateByClaimsSubReqDto,
        AtValidateReqDto,
    },
    repository::Repository,
};
use fip_common::common_error::CommonError;
use uuid::Uuid;

#[derive(Debug)]
pub struct Service {
    config: Config,
    repository: Repository,
}

impl Service {
    pub fn new(config: Config, repository: Repository) -> Self {
        Self { config, repository }
    }
}

impl Service {
    #[tracing::instrument]
    pub async fn delete(&self, req: &AtDeleteReqDto) -> Result<AtResDto, CommonError> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        let _ = self.repository.delete(&req.claims_jti).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn delete_by_claims_sub(
        &self,
        req: &AtDeleteByClaimsSubReqDto,
    ) -> Result<AtResDto, CommonError> {
        let res = self
            .repository
            .find_one_by_claims_sub(&req.claims_sub)
            .await?;
        let _ = self
            .repository
            .delete_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find(&self, _req: &AtFindReqDto) -> Result<Vec<AtResDto>, CommonError> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(|model| model.into()).collect())
    }

    #[tracing::instrument]
    pub async fn find_one(&self, req: &AtFindOneReqDto) -> Result<AtResDto, CommonError> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find_one_by_claims_sub(
        &self,
        req: &AtFindOneByClaimsSubReqDto,
    ) -> Result<AtResDto, CommonError> {
        let res = self
            .repository
            .find_one_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn save(&self, req: &AtSaveReqDto) -> Result<AtResDto, CommonError> {
        let mut model: Model = req.into();
        model.claims_jti = Uuid::new_v4().to_string().to_uppercase();
        let _ = self.repository.save(&model).await?;
        Ok(model.into())
    }

    #[tracing::instrument]
    pub async fn update(&self, req: &AtUpdateReqDto) -> Result<AtResDto, CommonError> {
        let model = req.into();
        let _ = self.repository.update(&model).await?;
        Ok(model.into())
    }

    #[tracing::instrument]
    pub async fn validate(&self, req: &AtValidateReqDto) -> Result<AtResDto, CommonError> {
        let res = self.repository.validate(&req.claims_jti).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn validate_by_claims_sub(
        &self,
        req: &AtValidateByClaimsSubReqDto,
    ) -> Result<AtResDto, CommonError> {
        let res = self
            .repository
            .validate_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }
}
