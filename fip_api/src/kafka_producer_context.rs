use rdkafka::{
    client::ClientContext, config::RDKafkaLogLevel, error::KafkaError, statistics::Statistics,
};

// A context can be used to change the behavior of producers and producers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
#[derive(Debug)]
pub struct KafkaProducerContext;

impl ClientContext for KafkaProducerContext {
    fn log(&self, level: RDKafkaLogLevel, fac: &str, log_message: &str) {
        match level {
            RDKafkaLogLevel::Emerg
            | RDKafkaLogLevel::Alert
            | RDKafkaLogLevel::Critical
            | RDKafkaLogLevel::Error => {
                tracing::error!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            RDKafkaLogLevel::Warning => {
                tracing::warn!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            RDKafkaLogLevel::Notice => {
                tracing::info!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            RDKafkaLogLevel::Info => {
                tracing::info!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
            RDKafkaLogLevel::Debug => {
                tracing::debug!(target: "librdkafka", "librdkafka: {} {}", fac, log_message)
            }
        }
    }

    fn stats(&self, statistics: Statistics) {
        tracing::info!("Client stats: {:?}", statistics);
    }

    fn error(&self, error: KafkaError, reason: &str) {
        tracing::error!("librdkafka: {}: {}", error, reason);
    }
}

// impl ProducerContext for KafkaProducerContext {
//     // type DeliveryOpaque: IntoOpaque;
//     // fn delivery(&self, delivery_result: &DeliveryResult<'_>, delivery_opaque: Self::DeliveryOpaque);
// }
