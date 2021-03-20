use crate::{
    common::sub::Sub,
    rt::{
        config::Config,
        proto::{
            client::{RtFindOneReqDto, RtResDto},
            server::rt_server::Rt,
        },
        service::Service,
    },
};
// use fip_common::common_opentelemetry::MetadataMap;
use tonic::{Request, Response, Status};
// use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Debug)]
pub struct Controller {
    config: Config,
    service: Service,
}

impl Controller {
    pub fn new(config: Config, service: Service) -> Self {
        Self { config, service }
    }
}

#[tonic::async_trait]
impl Rt for Controller {
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one(
        &self,
        request: Request<RtFindOneReqDto>,
    ) -> Result<Response<RtResDto>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = tracing::Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let sub = Self::sub(&request)?;
        let res = self.service.find_one(req, sub).await?;
        Ok(Response::new(res))
    }
}

impl Sub for Controller {}
