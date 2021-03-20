use crate::{
    config::Config,
    model::Model,
    proto::{
        JwksDeleteReqDto, JwksFindOneRandomReqDto, JwksFindOneReqDto, JwksFindReqDto, JwksResDto,
        JwksSaveReqDto, JwksUpdateReqDto,
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
    pub async fn delete(&self, req: &JwksDeleteReqDto) -> Result<JwksResDto, CommonError> {
        let res = self.repository.find_one(&req.id).await?;
        let _ = self.repository.delete(&req.id).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find(&self, _req: &JwksFindReqDto) -> Result<Vec<JwksResDto>, CommonError> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(|model| model.into()).collect())
    }

    #[tracing::instrument]
    pub async fn find_one(&self, req: &JwksFindOneReqDto) -> Result<JwksResDto, CommonError> {
        let res = self.repository.find_one(&req.id).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find_one_random(
        &self,
        _req: &JwksFindOneRandomReqDto,
    ) -> Result<JwksResDto, CommonError> {
        let res = self.repository.find_one_random().await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn save(&self, req: &JwksSaveReqDto) -> Result<JwksResDto, CommonError> {
        let mut model: Model = req.into();
        model.id = Uuid::new_v4().to_string().to_uppercase();
        let _ = self.repository.save(&model).await?;
        Ok(model.into())
    }

    #[tracing::instrument]
    pub async fn update(&self, req: &JwksUpdateReqDto) -> Result<JwksResDto, CommonError> {
        let model = req.into();
        let _ = self.repository.update(&model).await?;
        Ok(model.into())
    }
}
