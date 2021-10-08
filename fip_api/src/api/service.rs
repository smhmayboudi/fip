use crate::api::{
    config::Config,
    proto::server::{ApiFindOneReq, ApiRes},
};
use fip_common::common_error::CommonError;

/// TODO: documentation
#[derive(Debug)]
pub struct Service {
    config: Config,
}

impl Service {
    /// TODO: documentation
    pub fn new(config: Config) -> Self {
        Service { config }
    }
}

impl Service {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, req: &ApiFindOneReq) -> Result<ApiRes, CommonError> {
        Ok(ApiRes { id: req.id.clone() })
    }
}
