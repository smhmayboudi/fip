#![allow(trivial_casts)]

use crate::{
    config::Config,
    proto::{
        user_server::User, UserDeleteByCellphoneReq, UserDeleteByUsernameReq, UserDeleteReq,
        UserFindOneByCellphoneReq, UserFindOneByUsernameReq, UserFindOneReq, UserFindReq, UserRes,
        UserSaveReq, UserUpdateReq,
    },
    service::Service,
};
use fip_common::common_opentelemetry::MetadataMap;
use futures::Stream;
use std::pin::Pin;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use tracing::Span;
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
impl User for Controller {
    type FindStream = Pin<Box<dyn Stream<Item = Result<UserRes, Status>> + Send + Sync>>;

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete(&self, request: Request<UserDeleteReq>) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete_by_cellphone(
        &self,
        request: Request<UserDeleteByCellphoneReq>,
    ) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete_by_cellphone(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn delete_by_username(
        &self,
        request: Request<UserDeleteByUsernameReq>,
    ) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.delete_by_username(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find(
        &self,
        request: Request<UserFindReq>,
    ) -> Result<Response<Self::FindStream>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
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
        request: Request<UserFindOneReq>,
    ) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one_by_cellphone(
        &self,
        request: Request<UserFindOneByCellphoneReq>,
    ) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one_by_cellphone(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn find_one_by_username(
        &self,
        request: Request<UserFindOneByUsernameReq>,
    ) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.find_one_by_username(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn save(&self, request: Request<UserSaveReq>) -> Result<Response<UserRes>, Status> {
        let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.extract(&MetadataMap(request.metadata()))
        });
        let span = Span::current();
        span.set_parent(parent_context);
        let req = request.get_ref();
        let res = self.service.save(req).await?;
        Ok(Response::new(res))
    }

    #[tracing::instrument(fields(otel.kind = "server"))]
    async fn update(&self, request: Request<UserUpdateReq>) -> Result<Response<UserRes>, Status> {
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
