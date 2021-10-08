use crate::{
    config::Config,
    model::Model,
    proto::{
        UserDeleteByCellphoneReq, UserDeleteByUsernameReq, UserDeleteReq,
        UserFindOneByCellphoneReq, UserFindOneByUsernameReq, UserFindOneReq, UserFindReq, UserRes,
        UserSaveReq, UserUpdateReq,
    },
    repository::Repository,
};
use fip_common::common_error::CommonError;
use uuid::Uuid;

/// TODO: documentation
#[derive(Debug)]
pub struct Service {
    config: Config,
    repository: Repository,
}

impl Service {
    /// TODO: documentation
    pub fn new(config: Config, repository: Repository) -> Self {
        Self { config, repository }
    }
}

impl Service {
    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete(&self, req: &UserDeleteReq) -> Result<UserRes, CommonError> {
        let res = self.repository.find_one(&req.id).await?;
        let _ = self.repository.delete(&req.id).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete_by_cellphone(
        &self,
        req: &UserDeleteByCellphoneReq,
    ) -> Result<UserRes, CommonError> {
        let res = self
            .repository
            .find_one_by_cellphone(&req.cellphone)
            .await?;
        let _ = self.repository.delete_by_cellphone(&req.cellphone).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete_by_username(
        &self,
        req: &UserDeleteByUsernameReq,
    ) -> Result<UserRes, CommonError> {
        let res = self.repository.find_one_by_username(&req.username).await?;
        let _ = self.repository.delete_by_username(&req.username).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find(&self, _req: &UserFindReq) -> Result<Vec<UserRes>, CommonError> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(|model| model.into()).collect())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one(&self, req: &UserFindOneReq) -> Result<UserRes, CommonError> {
        let res = self.repository.find_one(&req.id).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one_by_cellphone(
        &self,
        req: &UserFindOneByCellphoneReq,
    ) -> Result<UserRes, CommonError> {
        let res = self
            .repository
            .find_one_by_cellphone(&req.cellphone)
            .await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one_by_username(
        &self,
        req: &UserFindOneByUsernameReq,
    ) -> Result<UserRes, CommonError> {
        let res = self.repository.find_one_by_username(&req.username).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn save(&self, req: &UserSaveReq) -> Result<UserRes, CommonError> {
        let mut model: Model = req.into();
        model.id = Uuid::new_v4().to_string().to_uppercase();
        let _ = self.repository.save(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn update(&self, req: &UserUpdateReq) -> Result<UserRes, CommonError> {
        let model = req.into();
        let _ = self.repository.update(&model).await?;
        Ok(model.into())
    }
}
