use rdkafka::{
    client::ClientContext,
    config::RDKafkaLogLevel,
    consumer::{ConsumerContext, Rebalance},
    error::{KafkaError, KafkaResult},
    statistics::Statistics,
    topic_partition_list::TopicPartitionList,
    util::Timeout,
};
use std::time::Duration;

// A context can be used to change the behavior of producers and consumers by adding callbacks
// that will be executed by librdkafka.
// This particular context sets up custom callbacks to log rebalancing events.
#[derive(Debug)]
pub struct KafkaConsumerContext;

/// TODO: documentation
impl ClientContext for KafkaConsumerContext {
    /// TODO: documentation
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

    /// TODO: documentation
    fn stats(&self, statistics: Statistics) {
        tracing::info!("Client stats: {:?}", statistics);
    }

    /// TODO: documentation
    fn error(&self, err: KafkaError, reason: &str) {
        tracing::error!("librdkafka: {}: {}", err, reason);
    }
}

/// TODO: documentation
impl ConsumerContext for KafkaConsumerContext {
    // /// TODO: documentation
    // fn rebalance(
    //     &self,
    //     native_client: &NativeClient,
    //     err: RDKafkaRespErr,
    //     tpl: &mut TopicPartitionList,
    // ) {
    //     tracing::info!("rebalance");
    // }

    /// TODO: documentation
    fn pre_rebalance(&self, rebalance: &Rebalance) {
        tracing::info!("pre_rebalance, {:?}", rebalance);
    }

    /// TODO: documentation
    fn post_rebalance(&self, rebalance: &Rebalance) {
        tracing::info!("post_rebalance, {:?}", rebalance);
    }

    /// TODO: documentation
    fn commit_callback(&self, result: KafkaResult<()>, _offsets: &TopicPartitionList) {
        tracing::info!("commit_callback, {:?}", result);
    }

    /// TODO: documentation
    fn main_queue_min_poll_interval(&self) -> Timeout {
        tracing::info!("main_queue_min_poll_interval");
        Timeout::After(Duration::from_secs(1))
    }

    /// TODO: documentation
    fn message_queue_nonempty_callback(&self) {
        tracing::info!("message_queue_nonempty_callback");
    }
}
