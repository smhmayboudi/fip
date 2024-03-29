use crate::{
    common::intercepted_client::InterceptedClient,
    user::{
        config::Config,
        proto::client::{
            user_client::UserClient, UserDeleteByCellphoneReq, UserDeleteByUsernameReq,
            UserDeleteReq, UserFindOneByCellphoneReq, UserFindOneByUsernameReq, UserFindOneReq,
            UserFindReq, UserRes, UserSaveReq, UserUpdateReq,
        },
    },
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
    pub async fn user_client_connect(&self) -> Result<UserClient<InterceptedClient>, Error> {
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
        Ok(UserClient::new(intercepted_client))
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, req: &UserDeleteReq, sub: &str) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
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
    pub async fn delete_by_cellphone(
        &self,
        req: &UserDeleteByCellphoneReq,
        sub: &str,
    ) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.delete_by_cellphone(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete_by_username(
        &self,
        req: &UserDeleteByUsernameReq,
        sub: &str,
    ) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.delete_by_username(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find(&self, req: &UserFindReq, sub: &str) -> Result<Vec<UserRes>, Status> {
        let mut client = self.user_client_connect().await?;
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
        let mut res = Vec::default();
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
    pub async fn find_one(&self, req: &UserFindOneReq, sub: &str) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
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
    pub async fn find_one_by_cellphone(
        &self,
        req: &UserFindOneByCellphoneReq,
        sub: &str,
    ) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.find_one_by_cellphone(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_username(
        &self,
        req: &UserFindOneByUsernameReq,
        sub: &str,
    ) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()));
        });
        let res = client.find_one_by_username(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _span_trace = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    /// TODO: documentation
    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, req: &UserSaveReq, sub: &str) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
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
    pub async fn update(&self, req: &UserUpdateReq, sub: &str) -> Result<UserRes, Status> {
        let mut client = self.user_client_connect().await?;
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
}
