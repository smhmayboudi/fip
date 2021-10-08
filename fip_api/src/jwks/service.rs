use crate::{
    common::intercepted_client::InterceptedClient,
    jwks::{
        config::Config,
        proto::client::{
            jwks_client::JwksClient, JwksDeleteReq, JwksFindOneRandomReq, JwksFindOneReq,
            JwksFindReq, JwksRes, JwksSaveReq, JwksUpdateReq,
        },
    },
};
use fip_common::{common_error::CommonError, common_opentelemetry::MetadataMapMut};
use tonic::{transport::Channel, Request, Status};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
#[derive(Debug)]
pub struct Service {
    config: Config,
}

impl Service {
    /// TODO: documentation
    pub fn new(config: Config) -> Self {
        Service { config }
    }
}

impl Service {
    /// TODO: documentation
    pub async fn jwks_client_connect(&self) -> Result<JwksClient<InterceptedClient>, CommonError> {
        let channel = Channel::from_shared(self.config.endpoint())
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?
            .connect()
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })?;
        let intercepted_client = InterceptedClient::new(channel);
        Ok(JwksClient::new(intercepted_client))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, req: &JwksDeleteReq, _sub: &str) -> Result<JwksRes, Status> {
        let mut client = self.jwks_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.delete(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find(&self, req: &JwksFindReq, _sub: &str) -> Result<Vec<JwksRes>, Status> {
        let mut client = self.jwks_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.find(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        let mut stream = res.into_inner();
        let mut res = Vec::default();
        while let Some(r) = stream.message().await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })? {
            res.push(r);
        }
        Ok(res)
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, req: &JwksFindOneReq, _sub: &str) -> Result<JwksRes, Status> {
        let mut client = self.jwks_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.find_one(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_random(
        &self,
        req: &JwksFindOneRandomReq,
        _sub: &str,
    ) -> Result<JwksRes, Status> {
        let mut client = self.jwks_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.find_one_random(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, req: &JwksSaveReq, _sub: &str) -> Result<JwksRes, Status> {
        let mut client = self.jwks_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.save(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn update(&self, req: &JwksUpdateReq, _sub: &str) -> Result<JwksRes, Status> {
        let mut client = self.jwks_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.update(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }
}
