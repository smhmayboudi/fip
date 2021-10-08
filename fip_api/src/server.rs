use crate::{
    api::controller::Controller, api::proto::server::api_server::ApiServer as ProtoApiServer,
    api::server::Server as ApiServer, at::server::Server as AtServer,
    auth::server::Server as AuthServer, config::Config, jwks::server::Server as JwksServer,
    rt::server::Server as RtServer, user::server::Server as UserServer,
};
use anyhow::Result;
use tonic::transport::Server as TonicServer;
use tonic_health::server::HealthReporter;
use tracing::Level;
// use tracing::Span;
// use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
#[derive(Debug)]
pub struct Server {}

impl Server {
    /// This function (somewhat improbably) flips the status of a service every second, in order
    /// that the effect of `tonic_health::HealthReporter::watch` can be easily observed.
    pub async fn check(mut reporter: HealthReporter) {
        let mut iter = 0u64;
        loop {
            iter += 1;
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            if iter % 2 == 0 {
                reporter.set_serving::<ProtoApiServer<Controller>>().await;
            } else {
                reporter
                    .set_not_serving::<ProtoApiServer<Controller>>()
                    .await;
            };
        }
    }

    /// TODO: documentation
    pub async fn init(config: &Config) -> Result<()> {
        let (mut health_reporter, health_server) = tonic_health::server::health_reporter();
        health_reporter
            .set_serving::<ProtoApiServer<Controller>>()
            .await;
        let _ = tokio::spawn(Self::check(health_reporter));

        TonicServer::builder()
            // .trace_fn(|header_map| {
            //     tracing::span!(
            //         target: "app_spans",
            //         parent: {
            //             let parent_context = opentelemetry::global::get_text_map_propagator(|propagator| {
            //                 propagator.extract(&tonic::metadata::MetadataMap::from_headers(
            //                     header_map.clone(),
            //                 ))
            //             });
            //             let span = Span::current();
            //             span.set_parent(parent_context);
            //             span
            //         },
            //         Level::INFO,
            //         env!("CARGO_PKG_NAME"),
            //         "otel.kind" = "server"
            //     )
            // })
            .trace_fn(|_header_map| {
                tracing::span!(Level::INFO, env!("CARGO_PKG_NAME"), "otel.kind" = "server")
            })
            .add_service(health_server)
            .add_service(ApiServer::new().into_inner())
            .add_service(AtServer::new().into_inner())
            .add_service(AuthServer::new().into_inner())
            .add_service(JwksServer::new().into_inner())
            .add_service(RtServer::new().into_inner())
            .add_service(UserServer::new().into_inner())
            .serve(config.socket_address())
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                anyhow::anyhow!(err)
            })
    }
}
