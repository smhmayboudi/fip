use crate::{
    at::{
        config::Config,
        proto::client::{
            at_client::AtClient, AtDeleteByClaimsSubReq, AtDeleteReq, AtFindOneByClaimsSubReq,
            AtFindOneReq, AtFindReq, AtRes, AtSaveReq, AtUpdateReq, AtValidateByClaimsSubReq,
            AtValidateReq,
        },
    },
    common::intercepted_client::InterceptedClient,
};
use fip_common::{error::Error, opentelemetry::MetadataMapMut};
use tonic::{transport::Channel, Request, Status};
use tracing::Span;
use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
#[derive(Debug)]
pub struct Service {
    config: Config,
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    #[must_use]
    pub const fn new(config: Config) -> Self {
        Self { config }
    }
}

/// TODO: documentation
impl Service {
    /// TODO: documentation
    ///
    /// # Errors
    /// TODO: documentation errors
    pub async fn at_client_connect(&self) -> Result<AtClient<InterceptedClient>, Error> {
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
        Ok(AtClient::new(intercepted_client))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, req: &AtDeleteReq, sub: &str) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.delete(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete_by_claims_sub(
        &self,
        req: &AtDeleteByClaimsSubReq,
        sub: &str,
    ) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.delete_by_claims_sub(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find(&self, req: &AtFindReq, sub: &str) -> Result<Vec<AtRes>, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.find(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        let mut stream = res.into_inner();
        let mut res = Vec::new();
        while let Some(r) = stream.message().await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })? {
            res.push(r);
        }
        Ok(res)
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(&self, req: &AtFindOneReq, sub: &str) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.find_one(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_claims_sub(
        &self,
        req: &AtFindOneByClaimsSubReq,
        sub: &str,
    ) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client
            .find_one_by_claims_sub(request)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                let _span_trace = tracing_error::SpanTrace::capture();
                err
            })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, req: &AtSaveReq, sub: &str) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.save(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn update(&self, req: &AtUpdateReq, sub: &str) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.update(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn validate(&self, req: &AtValidateReq, sub: &str) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.validate(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn validate_by_claims_sub(
        &self,
        req: &AtValidateByClaimsSubReq,
        sub: &str,
    ) -> Result<AtRes, Status> {
        let mut client = self.at_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client
            .validate_by_claims_sub(request)
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                let _span_trace = tracing_error::SpanTrace::capture();
                err
            })?;
        Ok(res.into_inner())
    }
}
