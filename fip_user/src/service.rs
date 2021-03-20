use crate::{
    config::Config,
    model::Model,
    proto::{
        UserDeleteByCellphoneReqDto, UserDeleteByUsernameReqDto, UserDeleteReqDto,
        UserFindOneByCellphoneReqDto, UserFindOneByUsernameReqDto, UserFindOneReqDto,
        UserFindReqDto, UserResDto, UserSaveReqDto, UserUpdateReqDto,
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
    pub async fn delete(&self, req: &UserDeleteReqDto) -> Result<UserResDto, CommonError> {
        let res = self.repository.find_one(&req.id).await?;
        let _ = self.repository.delete(&req.id).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn delete_by_cellphone(
        &self,
        req: &UserDeleteByCellphoneReqDto,
    ) -> Result<UserResDto, CommonError> {
        let res = self
            .repository
            .find_one_by_cellphone(&req.cellphone)
            .await?;
        let _ = self.repository.delete_by_cellphone(&req.cellphone).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn delete_by_username(
        &self,
        req: &UserDeleteByUsernameReqDto,
    ) -> Result<UserResDto, CommonError> {
        let res = self.repository.find_one_by_username(&req.username).await?;
        let _ = self.repository.delete_by_username(&req.username).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find(&self, _req: &UserFindReqDto) -> Result<Vec<UserResDto>, CommonError> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(|model| model.into()).collect())
    }

    #[tracing::instrument]
    pub async fn find_one(&self, req: &UserFindOneReqDto) -> Result<UserResDto, CommonError> {
        let res = self.repository.find_one(&req.id).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find_one_by_cellphone(
        &self,
        req: &UserFindOneByCellphoneReqDto,
    ) -> Result<UserResDto, CommonError> {
        let res = self
            .repository
            .find_one_by_cellphone(&req.cellphone)
            .await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn find_one_by_username(
        &self,
        req: &UserFindOneByUsernameReqDto,
    ) -> Result<UserResDto, CommonError> {
        let res = self.repository.find_one_by_username(&req.username).await?;
        Ok(res.into())
    }

    #[tracing::instrument]
    pub async fn save(&self, req: &UserSaveReqDto) -> Result<UserResDto, CommonError> {
        let mut model: Model = req.into();
        model.id = Uuid::new_v4().to_string().to_uppercase();
        let _ = self.repository.save(&model).await?;
        Ok(model.into())
    }

    #[tracing::instrument]
    pub async fn update(&self, req: &UserUpdateReqDto) -> Result<UserResDto, CommonError> {
        let model = req.into();
        let _ = self.repository.update(&model).await?;
        Ok(model.into())
    }
}
