use crate::{
    common::sub::Sub,
    user::{
        config::Config,
        proto::{
            client::{UserFindOneReq, UserRes},
            server::user_server::User,
        },
        service::Service,
    },
};
// use fip_common::opentelemetry::MetadataMap;
use tonic::{Request, Response, Status};
// use tracing::Span;
// use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
#[allow(dead_code)]
#[derive(Debug)]
pub struct Controller {
    config: Config,
    service: Service,
}

/// TODO: documentation
impl Controller {
    /// TODO: documentation
    #[must_use]
    pub const fn new(config: Config, service: Service) -> Self {
        Self { config, service }
    }
}

#[tonic::async_trait]
impl User for Controller {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one(
        &self,
        request: Request<UserFindOneReq>,
    ) -> Result<Response<UserRes>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let sub = Self::sub(&request)?;
        let res = self.service.find_one(req, sub).await?;
        Ok(Response::new(res))
    }
}

/// TODO: documentation
impl Sub for Controller {}
