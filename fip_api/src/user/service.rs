use crate::{
    common::intercepted_client::InterceptedClient,
    user::{
        config::Config,
        proto::client::{
            user_client::UserClient, UserDeleteByCellphoneReqDto, UserDeleteByUsernameReqDto,
            UserDeleteReqDto, UserFindOneByCellphoneReqDto, UserFindOneByUsernameReqDto,
            UserFindOneReqDto, UserFindReqDto, UserResDto, UserSaveReqDto, UserUpdateReqDto,
        },
    },
};
use fip_common::{common_error::CommonError, common_opentelemetry::MetadataMapMut};
use tonic::{transport::Channel, Request, Status};
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Debug)]
pub struct Service {
    config: Config,
}

impl Service {
    pub fn new(config: Config) -> Self {
        Service { config }
    }
}

impl Service {
    pub async fn user_client_connect(&self) -> Result<UserClient<InterceptedClient>, CommonError> {
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

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete(&self, req: &UserDeleteReqDto, _sub: &str) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
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

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete_by_cellphone(
        &self,
        req: &UserDeleteByCellphoneReqDto,
        _sub: &str,
    ) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.delete_by_cellphone(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn delete_by_username(
        &self,
        req: &UserDeleteByUsernameReqDto,
        _sub: &str,
    ) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.delete_by_username(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find(&self, req: &UserFindReqDto, _sub: &str) -> Result<Vec<UserResDto>, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.find(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        let mut stream = res.into_inner();
        let mut res = Vec::new();
        while let Some(r) = stream.message().await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })? {
            res.push(r);
        }
        Ok(res)
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one(
        &self,
        req: &UserFindOneReqDto,
        _sub: &str,
    ) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
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

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_cellphone(
        &self,
        req: &UserFindOneByCellphoneReqDto,
        _sub: &str,
    ) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.find_one_by_cellphone(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn find_one_by_username(
        &self,
        req: &UserFindOneByUsernameReqDto,
        _sub: &str,
    ) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
        opentelemetry::global::get_text_map_propagator(|propagator| {
            propagator.inject_context(&context, &mut MetadataMapMut(request.metadata_mut()))
        });
        let res = client.find_one_by_username(request).await.map_err(|err| {
            tracing::error!("{:?}", err);
            let _ = tracing_error::SpanTrace::capture();
            err
        })?;
        Ok(res.into_inner())
    }

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn save(&self, req: &UserSaveReqDto, _sub: &str) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
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

    #[tracing::instrument(fields(otel.kind = "client"))]
    pub async fn update(&self, req: &UserUpdateReqDto, _sub: &str) -> Result<UserResDto, Status> {
        let mut client = self.user_client_connect().await?;
        let mut request = Request::new(req.clone());
        let context = tracing::Span::current().context();
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
