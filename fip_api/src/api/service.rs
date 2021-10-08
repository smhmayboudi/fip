use crate::api::{
    config::Config,
    proto::server::{ApiFindOneReq, ApiRes},
};
use fip_common::error::Error;

/// TODO: documentation
#[derive(Debug)]
pub struct Service {
    config: Config,
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, req: &ApiFindOneReq) -> Result<ApiRes, Error> {
        Ok(ApiRes { id: req.id.clone() })
    }
}
