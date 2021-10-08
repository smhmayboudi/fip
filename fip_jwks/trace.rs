use crate::config::Config;
use anyhow::Result;
use opentelemetry::{
    runtime::Tokio,
    sdk::{
        trace::{self, Sampler},
        Resource,
    },
    KeyValue,
};
use opentelemetry_zipkin::Propagator;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_error::ErrorLayer;
use tracing_subscriber::{
    layer::SubscriberExt,
    {EnvFilter, Registry},
};
s
/// TODO: documentation
#[derive(Debug)]
pub struct Trace {}

impl Trace {
/// TODO: documentation
    pub fn init(config: &Config) -> Result<()> {
        opentelemetry::global::set_text_map_propagator(Propagator::new());
        let tracer = opentelemetry_zipkin::new_pipeline()
            // .from_env()
            .with_service_name(config.app_name())
            // .with_tags(vec![KeyValue::new("version", config.app_version())])
            .with_trace_config(
                trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_resource(Resource::new(vec![KeyValue::new(
                        "version",
                        config.app_version(),
                    )])),
            )
            .install_batch(Tokio)
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
        })
    }
}
