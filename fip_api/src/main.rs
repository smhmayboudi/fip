pub mod api;
pub mod at;
pub mod auth;
pub mod common;
pub mod config;
pub mod jwks;
pub mod rt;
pub mod server;
pub mod user;

use crate::{config::Config, server::Server};
use opentelemetry::sdk::propagation::TraceContextPropagator;
use opentelemetry::KeyValue;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    layer::SubscriberExt,
    {EnvFilter, Registry},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::default();

    opentelemetry::global::set_text_map_propagator(TraceContextPropagator::new());
    let (tracer, _uninstall) = opentelemetry_jaeger::new_pipeline()
        .from_env()
        .with_service_name(&config.app_name())
        .with_tags(vec![KeyValue::new("version", config.app_version())])
        .install()
        .map_err(|err| {
            tracing::error!("{:?}", err);
            anyhow::anyhow!(err)
        })?;
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    // let file_appender = tracing_appender::rolling::daily("log", "prefix.log");
    // let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    let formatting_layer = BunyanFormattingLayer::new(config.app_name(), non_blocking_writer);
    let subscriber = Registry::default()
        .with(EnvFilter::from_default_env())
        .with(JsonStorageLayer)
        .with(formatting_layer)
        .with(opentelemetry)
        .with(ErrorLayer::default());
    tracing::subscriber::set_global_default(subscriber).map_err(|err| {
        tracing::error!("{:?}", err);
        anyhow::anyhow!(err)
    })?;

    Server::init(config.clone()).await
}
