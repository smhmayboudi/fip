#![allow(trivial_casts)]

use crate::{
    config::Config,
    proto::{
        at_server::At, AtDeleteByClaimsSubReq, AtDeleteReq, AtFindOneByClaimsSubReq, AtFindOneReq,
        AtFindReq, AtRes, AtSaveReq, AtUpdateReq, AtValidateByClaimsSubReq, AtValidateReq,
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
impl At for Controller {
    type FindStream = ReceiverStream<Result<AtRes, Status>>;

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete(&self, request: Request<AtDeleteReq>) -> Result<Response<AtRes>, Status> {
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
    async fn delete_by_claims_sub(
        &self,
        request: Request<AtDeleteByClaimsSubReq>,
    ) -> Result<Response<AtRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete_by_claims_sub(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find(
        &self,
        request: Request<AtFindReq>,
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
    async fn find_one(&self, request: Request<AtFindOneReq>) -> Result<Response<AtRes>, Status> {
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
    async fn find_one_by_claims_sub(
        &self,
        request: Request<AtFindOneByClaimsSubReq>,
    ) -> Result<Response<AtRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one_by_claims_sub(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn save(&self, request: Request<AtSaveReq>) -> Result<Response<AtRes>, Status> {
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
    async fn update(&self, request: Request<AtUpdateReq>) -> Result<Response<AtRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.update(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn validate(&self, request: Request<AtValidateReq>) -> Result<Response<AtRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.validate(req).await?;
        Ok(Response::new(res))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn validate_by_claims_sub(
        &self,
        request: Request<AtValidateByClaimsSubReq>,
    ) -> Result<Response<AtRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.validate_by_claims_sub(req).await?;
        Ok(Response::new(res))
    }
}
