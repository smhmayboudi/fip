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
use fip_common::error::Error;
use uuid::Uuid;

/// TODO: documentation
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
    pub async fn delete(&self, req: &UserDeleteReq) -> Result<UserRes, Error> {
        let res = self.repository.find_one(&req.id).await?;
        let _rows_affected = self.repository.delete(&req.id).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete_by_cellphone(
        &self,
        req: &UserDeleteByCellphoneReq,
    ) -> Result<UserRes, Error> {
        let res = self
            .repository
            .find_one_by_cellphone(&req.cellphone)
            .await?;
        let _rows_affected = self.repository.delete_by_cellphone(&req.cellphone).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn delete_by_username(
        &self,
        req: &UserDeleteByUsernameReq,
    ) -> Result<UserRes, Error> {
        let res = self.repository.find_one_by_username(&req.username).await?;
        let _rows_affected = self.repository.delete_by_username(&req.username).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find(&self, req: &UserFindReq) -> Result<Vec<UserRes>, Error> {
        let res = self.repository.find().await?;
        Ok(res.into_iter().map(|model| model.into()).collect())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one(&self, req: &UserFindOneReq) -> Result<UserRes, Error> {
        let res = self.repository.find_one(&req.id).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn find_one_by_cellphone(
        &self,
        req: &UserFindOneByCellphoneReq,
    ) -> Result<UserRes, Error> {
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
    ) -> Result<UserRes, Error> {
        let res = self.repository.find_one_by_username(&req.username).await?;
        Ok(res.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn save(&self, req: &UserSaveReq) -> Result<UserRes, Error> {
        let mut model: Model = req.into();
        model.id = Uuid::new_v4().to_string().to_uppercase();
        let _rows_affected = self.repository.save(&model).await?;
        Ok(model.into())
    }

    /// TODO: documentation
    #[tracing::instrument]
    pub async fn update(&self, req: &UserUpdateReq) -> Result<UserRes, Error> {
        let model = req.into();
        let _rows_affected = self.repository.update(&model).await?;
        Ok(model.into())
    }
}
