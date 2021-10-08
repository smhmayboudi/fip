use crate::api::{
    config::Config,
    proto::server::{api_server::Api, ApiFindOneReq, ApiRes},
    service::Service,
};
// use fip_common::common_opentelemetry::MetadataMap;
use tonic::{Request, Response, Status};
// use tracing::Spantracing::Span;
// use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
#[derive(Debug)]
pub struct Controller {
    config: Config,
    service: Service,
}

impl Controller {
    /// TODO: documentation
    pub fn new(config: Config, service: Service) -> Self {
        Self { config, service }
    }
}

#[tonic::async_trait]
impl Api for Controller {
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one(&self, request: Request<ApiFindOneReq>) -> Result<Response<ApiRes>, Status> {
        // TODO: test features for fip_api
        // #[cfg(feature = "attributes")]
        // tracing::error!("features = [\"attributes\"]");

        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one(req).await?;
        Ok(Response::new(res))
    }
}
