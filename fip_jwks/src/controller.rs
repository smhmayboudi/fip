#![allow(trivial_casts)]

use crate::{
    config::Config,
    proto::{
        jwks_server::Jwks, JwksDeleteReq, JwksFindOneRandomReq, JwksFindOneReq, JwksFindReq,
        JwksRes, JwksSaveReq, JwksUpdateReq,
    },
    service::Service,
};
use fip_common::opentelemetry::MetadataMap;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

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
impl Jwks for Controller {
    type FindStream = ReceiverStream<Result<JwksRes, Status>>;

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete(&self, request: Request<JwksDeleteReq>) -> Result<Response<JwksRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find(
        &self,
        request: Request<JwksFindReq>,
    ) -> Result<Response<Self::FindStream>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find(req).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let _ok = tokio::spawn(async move {
            for res in &res {
                tx.send(Ok(res.clone())).await.unwrap_or_else(|err| {
                    tracing::error!("receiver dropped, {:?}", err);
                });
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one(
        &self,
        request: Request<JwksFindOneReq>,
    ) -> Result<Response<JwksRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one_random(
        &self,
        request: Request<JwksFindOneRandomReq>,
    ) -> Result<Response<JwksRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one_random(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn save(&self, request: Request<JwksSaveReq>) -> Result<Response<JwksRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.save(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn update(&self, request: Request<JwksUpdateReq>) -> Result<Response<JwksRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.update(req).await?;
        Ok(Response::new(res))
    }
}
