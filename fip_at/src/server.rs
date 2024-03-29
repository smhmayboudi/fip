use crate::{
    config::Config, controller::Controller, proto::at_server::AtServer, repository::Repository,
    service::Service,
};
use anyhow::Result;
use tokio::time::Duration;
use tonic::transport::Server as TonicServer;
use tonic_health::server::HealthReporter;
// use tracing::{Level, Span};
// use tracing_opentelemetry::OpenTelemetrySpanExt;

/// TODO: documentation
#[derive(Clone, Debug)]
pub struct Server {
    inner: AtServer<Controller>,
}

/// TODO: documentation
impl Server {
    /// This function (somewhat improbably) flips the status of a service every second, in order
    /// that the effect of `tonic_health::HealthReporter::watch` can be easily observed.
    pub async fn check(mut reporter: HealthReporter) {
        let mut iter = 0_u64;
        loop {
            iter += 1;
            tokio::time::sleep(Duration::from_secs(1)).await;
            if iter % 2 == 0 {
                reporter.set_serving::<AtServer<Controller>>().await;
            } else {
                reporter.set_not_serving::<AtServer<Controller>>().await;
            };
        }
    }

    /// TODO: documentation
    pub async fn new() -> Self {
        let config = Config::new();
        let repository = Repository::new(config.clone()).await;
        let service = Service::new(config.clone(), repository);
        let controller = Controller::new(config, service);

        let server = AtServer::new(controller);
        Self { inner: server }
    }

    /// TODO: documentation
    ///
    /// # Errors
    ///
    /// TODO: documentation errors
    pub async fn init(config: &Config) -> Result<()> {
        let (mut health_reporter, health_server) = tonic_health::server::health_reporter();
        health_reporter.set_serving::<AtServer<Controller>>().await;
        let _ok = tokio::spawn(Self::check(health_reporter));

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
            .add_service(health_server)
            .add_service(Self::new().await.into_inner())
            .serve(config.socket_address())
            .await
            .map_err(|err| {
                tracing::error!("{:?}", err);
                anyhow::anyhow!(err)
            })
    }
}

/// TODO: documentation
impl Server {
    /// TODO: documentation
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub fn into_inner(self) -> AtServer<Controller> {
        self.inner
    }
}
