#![allow(trivial_casts)]

use crate::{
    config::Config,
    proto::{
        rt_server::Rt, RtDeleteByClaimsSubReqDto, RtDeleteReqDto, RtFindOneByClaimsSubReqDto,
        RtFindOneReqDto, RtFindReqDto, RtResDto, RtSaveReqDto, RtUpdateReqDto,
        RtValidateByClaimsSubReqDto, RtValidateReqDto,
    },
    service::Service,
};
use fip_common::common_opentelemetry::MetadataMap;
use futures::Stream;
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing_opentelemetry::OpenTelemetrySpanExt;

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
    type FindStream = Pin<Box<dyn Stream<Item = Result<RtResDto, Status>> + Send + Sync>>;

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete(&self, request: Request<RtDeleteReqDto>) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete_by_claims_sub(
        &self,
        request: Request<RtDeleteByClaimsSubReqDto>,
    ) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete_by_claims_sub(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find(
        &self,
        request: Request<RtFindReqDto>,
    ) -> Result<Response<Self::FindStream>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find(req).await?;

        let (tx, rx) = tokio::sync::mpsc::channel(4);
        let _ = tokio::spawn(async move {
            for res in &res {
                tx.send(Ok(res.clone())).await.unwrap();
            }
        });

        Ok(Response::new(
            Box::pin(ReceiverStream::new(rx)) as Self::FindStream
        ))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one(
        &self,
        request: Request<RtFindOneReqDto>,
    ) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one_by_claims_sub(
        &self,
        request: Request<RtFindOneByClaimsSubReqDto>,
    ) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one_by_claims_sub(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn save(&self, request: Request<RtSaveReqDto>) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.save(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn update(&self, request: Request<RtUpdateReqDto>) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.update(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn validate(
        &self,
        request: Request<RtValidateReqDto>,
    ) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.validate(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn validate_by_claims_sub(
        &self,
        request: Request<RtValidateByClaimsSubReqDto>,
    ) -> Result<Response<RtResDto>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = tracing::Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.validate_by_claims_sub(req).await?;
        Ok(Response::new(res))
    }
}
