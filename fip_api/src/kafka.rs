#![allow(dead_code)]
#![allow(missing_debug_implementations)]

use crate::{
    config::Config, kafka_consumer_context::KafkaConsumerContext,
    kafka_producer_context::KafkaProducerContext,
};
use rdkafka::{
    config::{ClientConfig, RDKafkaLogLevel},
    consumer::{CommitMode, Consumer, StreamConsumer},
    message::{Headers, Message, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
};
use std::{str::Utf8Error, time::Duration};
use uuid::Uuid;

/// TODO: documentation
pub struct Kafka {
    config: Config,
    consumer: StreamConsumer<KafkaConsumerContext>,
    producer: FutureProducer<KafkaProducerContext>,
}

/// TODO: documentation
impl Kafka {
    /// TODO: documentation
    ///
    /// # Panics
    /// TODO: documentation panics
    pub fn new(config: Config) -> Self {
        let context = KafkaConsumerContext;
        let consumer = ClientConfig::default()
            .set("auto.offset.reset", "earliest")
            .set("bootstrap.servers", &config.kafka_broker())
            .set("client.id", "fip-api-consumer")
            .set("debug", "all")
            .set("enable.auto.commit", "false")
            .set("enable.auto.offset.store", "false")
            // .set("group.id", &config.kafka_consumer_group())
            .set("group.id", "TEST")
            .set("group.instance.id", "consumer")
            // .set("statistics.interval.ms", "10000")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })
            .unwrap_or_else(|err| {
                panic!("{:?}", err);
            });

        let context = KafkaProducerContext;
        let producer = ClientConfig::default()
            .set("bootstrap.servers", &config.kafka_broker())
            .set("client.id", "fip-api-producer")
            .set("debug", "all")
            .set("enable.idempotence", "true")
            .set("max.in.flight.requests.per.connection", "1")
            .set_log_level(RDKafkaLogLevel::Debug)
            .create_with_context(context)
            .map_err(|err| {
                tracing::error!("{:?}", err);
                err
            })
            .unwrap_or_else(|err| {
                panic!("{:?}", err);
            });

        Kafka {
            config,
            consumer,
            producer,
        }
    }

    /// TODO: documentation
    ///
    /// # Errors
    /// TODO: documentation errors
    fn echo_message<M: Message>(msg: M) -> Result<(), Utf8Error> {
        let deserialize = |o| match o {
            None => Ok(""),
            Some(val) => Ok(std::str::from_utf8(val)?),
        };
        println!(
            "Consumed record from topic {:?} partition [{:?}] @ offset {:?} with key {:?} and value {:?}",
            msg.topic(),
            msg.partition(),
            msg.offset(),
            deserialize(msg.key())?,
            deserialize(msg.payload())?,
        );
        Ok(())
    }
}

/// TODO: documentation
impl Kafka {
    /// TODO: documentation
    pub fn process() {}

    /// TODO: documentation
    pub fn handle() {}

    // pub fn receive(&self, topic: &str) {
    /// TODO: documentation
    pub async fn receive(&self) {
        let topic = "TEST_REQ";

        self.consumer.subscribe(&[topic]).unwrap_or_else(|err| {
            panic!("{:?}", err);
        });

        loop {
            match self.consumer.recv().await {
                Ok(borrowed_message) => {
                    let key = match borrowed_message.key_view::<str>() {
                        None => "",
                        Some(Err(e)) => {
                            tracing::error!("Error while deserializing message key: {:?}", e);
                            ""
                        }
                        Some(Ok(s)) => s,
                    };
                    let payload = match borrowed_message.payload_view::<str>() {
                        None => "",
                        Some(Err(e)) => {
                            tracing::error!("Error while deserializing message payload: {:?}", e);
                            ""
                        }
                        Some(Ok(s)) => s,
                    };
                    tracing::info!("key: {:?}, payload: {:?}, topic: {:?}, partition: {:?}, offset: {:?}, timestamp: {:?}",
                          key, payload, borrowed_message.topic(), borrowed_message.partition(), borrowed_message.offset(), borrowed_message.timestamp());
                    if let Some(headers) = borrowed_message.headers() {
                        for i in 0..headers.count() {
                            let header = headers.get_as::<str>(i).unwrap_or_else(|err| {
                                panic!("{:?}", err);
                            });
                            tracing::info!("  Header {:#?}: {:?}", header.0, header.1);
                        }
                    }
                    let res = self
                        .consumer
                        .commit_message(&borrowed_message, CommitMode::Sync);
                    match res {
                        Err(e) => tracing::error!("Could not commit message: {:?} ", e),
                        Ok(()) => tracing::info!("commit message"),
                    };
                }
                Err(kafka_error) => {
                    tracing::error!(
                        "Could not receive and will not process message: {:?}",
                        kafka_error
                    );
                }
            }
        }
    }

    // pub fn send(&self, topic: &str, key: &str, payload: &str) -> DeliveryFuture {
    /// TODO: documentation
    pub async fn send(&self) {
        let topic = "TEST_REQ";
        let key = "user_id";
        let payload = "user_data";

        let cid = Uuid::new_v4();
        let cid = format!("{}", cid);
        let cid: &str = cid.as_ref();

        let rto = "TEST_RES";

        let headers = OwnedHeaders::default()
            .add("correlation_id", cid)
            .add("reply_to", rto);

        let record: FutureRecord<'_, str, str> = FutureRecord::to(topic)
            .headers(headers)
            .key(key)
            .payload(payload);

        let queue_timeout = Duration::from_secs(0);

        let produce_future = self.producer.send(record, queue_timeout);

        match produce_future.await {
            Ok((a, b)) => tracing::info!("partition: {:?}, offset: {:?}", a, b),
            Err((kafka_error, owned_message)) => {
                tracing::error!(
                    "kafka_error: {:?}, owned_message: {:?}",
                    kafka_error,
                    owned_message
                )
            }
        };
    }
}
