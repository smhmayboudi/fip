use crate::{
    auth::{
        config::Config,
        proto::server::{
            auth_server::Auth, AuthLoginReq, AuthLoginRes, AuthLogoutReq, AuthLogoutRes,
            AuthTokenReq, AuthTokenRes,
        },
        service::Service,
    },
    common::sub::Sub,
};
// use fip_common::opentelemetry::MetadataMap;
use tonic::{Request, Response, Status};
// use tracing::Span;
// use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
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
impl Auth for Controller {
    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn login(
        &self,
        request: Request<AuthLoginReq>,
    ) -> Result<Response<AuthLoginRes>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.login(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn logout(
        &self,
        request: Request<AuthLogoutReq>,
    ) -> Result<Response<AuthLogoutRes>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let sub = Self::sub(&request)?;
        let res = self.service.logout(req, sub).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn token(
        &self,
        request: Request<AuthTokenReq>,
    ) -> Result<Response<AuthTokenRes>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let sub = Self::sub(&request)?;
        let res = self.service.token(req, sub).await?;
        Ok(Response::new(res))
    }
}

/// TODO: documentation
impl Sub for Controller {}
