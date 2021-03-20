use crate::{
    config::Config,
    model::Model,
    proto::{
        RtDeleteByClaimsSubReqDto, RtDeleteReqDto, RtFindOneByClaimsSubReqDto, RtFindOneReqDto,
        RtFindReqDto, RtResDto, RtSaveReqDto, RtUpdateReqDto, RtValidateByClaimsSubReqDto,
        RtValidateReqDto,
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
    pub async fn delete(&self, req: &RtDeleteReqDto) -> Result<RtResDto, CommonError> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        let _e = self.repository.delete(&req.claims_jti).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn delete_by_claims_sub(
        &self,
        req: &RtDeleteByClaimsSubReqDto,
    ) -> Result<RtResDto, CommonError> {
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
    pub async fn find(&self, _req: &RtFindReqDto) -> Result<Vec<RtResDto>, CommonError> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(|model| model.into()).collect())
    }

    #[tracing::instrument]
    pub async fn find_one(&self, req: &RtFindOneReqDto) -> Result<RtResDto, CommonError> {
        let res = self.repository.find_one(&req.claims_jti).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find_one_by_claims_sub(
        &self,
        req: &RtFindOneByClaimsSubReqDto,
    ) -> Result<RtResDto, CommonError> {
        let res = self
            .repository
            .find_one_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn save(&self, req: &RtSaveReqDto) -> Result<RtResDto, CommonError> {
        let mut model: Model = req.into();
        model.claims_jti = Uuid::new_v4().to_string().to_uppercase();
        let _ = self.repository.save(&model).await?;
        Ok(model.into())
    }

    #[tracing::instrument]
    pub async fn update(&self, req: &RtUpdateReqDto) -> Result<RtResDto, CommonError> {
        let model = req.into();
        let _ = self.repository.update(&model).await?;
        Ok(model.into())
    }

    #[tracing::instrument]
    pub async fn validate(&self, req: &RtValidateReqDto) -> Result<RtResDto, CommonError> {
        let res = self.repository.validate(&req.claims_jti).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn validate_by_claims_sub(
        &self,
        req: &RtValidateByClaimsSubReqDto,
    ) -> Result<RtResDto, CommonError> {
        let res = self
            .repository
            .validate_by_claims_sub(&req.claims_sub)
            .await?;
        Ok(res.into())
    }
}
