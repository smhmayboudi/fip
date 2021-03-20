use crate::api::{
    config::Config,
    proto::server::{ApiFindOneReqDto, ApiResDto},
};
use fip_common::common_error::CommonError;

#[derive(Debug)]
pub struct Service {
    config: Config,
}

impl Service {
    pub fn new(config: Config) -> Self {
        Service { config }
    }
}

impl Service {
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, req: &ApiFindOneReqDto) -> Result<ApiResDto, CommonError> {
        Ok(ApiResDto { id: req.id.clone() })
    }
}
