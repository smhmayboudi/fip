use crate::{
    auth::{
        config::Config,
        proto::server::{
            auth_server::Auth, AuthLoginReqDto, AuthLoginResDto, AuthLogoutReqDto,
            AuthLogoutResDto, AuthTokenReqDto, AuthTokenResDto,
        },
        service::Service,
    },
    common::sub::Sub,
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
impl Auth for Controller {
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn login(
        &self,
        request: Request<AuthLoginReqDto>,
    ) -> Result<Response<AuthLoginResDto>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = tracing::Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.login(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn logout(
        &self,
        request: Request<AuthLogoutReqDto>,
    ) -> Result<Response<AuthLogoutResDto>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = tracing::Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let sub = Self::sub(&request)?;
        let res = self.service.logout(req, sub).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn token(
        &self,
        request: Request<AuthTokenReqDto>,
    ) -> Result<Response<AuthTokenResDto>, Status> {
        // let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
        //     propagator.extract(&MetadataMap(request.metadata()))
        // });
        // let span = tracing::Span::current();
        // span.set_parent(parent_context);
        let req = request.get_ref();
        let sub = Self::sub(&request)?;
        let res = self.service.token(req, sub).await?;
        Ok(Response::new(res))
    }
}

impl Sub for Controller {}
