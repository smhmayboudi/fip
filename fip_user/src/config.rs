use fip_common::common_const::{FIP, GROUP, REQ, RES, USER, USER_DB_SQLITE};
use serde::Deserialize;
use std::net::SocketAddr;

fn default_app_name() -> String {
    env!("CARGO_PKG_NAME").into()
}

fn default_app_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

fn default_database_url() -> String {
    USER_DB_SQLITE.into()
}

fn default_endpoint() -> String {
    format!("http://{}", default_socket_address())
}

fn default_kafka_broker() -> String {
    "127.0.0.1:9092".into()
}

fn default_kafka_consumer_group() -> String {
    format!("{}_{}_{}", FIP, default_token(), GROUP)
}

fn default_kafka_topic_req() -> String {
    format!("{}_{}_{}", FIP, default_token(), REQ)
}

fn default_kafka_topic_res() -> String {
    format!("{}_{}_{}", FIP, default_token(), RES)
}

fn default_socket_address() -> SocketAddr {
    ([0, 0, 0, 0], 8080).into()
}

fn default_token() -> String {
    USER.into()
}

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_app_name")]
    app_name: String,
    #[serde(default = "default_app_version")]
    app_version: String,
    #[serde(default = "default_database_url")]
    database_url: String,
    #[serde(default = "default_endpoint")]
    endpoint: String,
    #[serde(default = "default_kafka_broker")]
    kafka_broker: String,
    #[serde(default = "default_kafka_consumer_group")]
    kafka_consumer_group: String,
    #[serde(default = "default_kafka_topic_req")]
    kafka_topic_req: String,
    #[serde(default = "default_kafka_topic_res")]
    kafka_topic_res: String,
    #[serde(default = "default_socket_address")]
    socket_address: SocketAddr,
    #[serde(default = "default_token")]
    token: String,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Server {
    pub fn app_name(&self) -> String {
        self.app_name.clone()
    }

    pub fn app_version(&self) -> String {
        self.app_version.clone()
    }

    pub fn database_url(&self) -> String {
        self.database_url.clone()
    }

    pub fn endpoint(&self) -> String {
        self.endpoint.clone()
    }

    pub fn kafka_broker(&self) -> String {
        self.kafka_broker.clone()
    }

    pub fn kafka_consumer_group(&self) -> String {
        self.kafka_consumer_group.clone()
    }

    pub fn kafka_topic_req(&self) -> String {
        self.kafka_topic_req.clone()
    }

    pub fn kafka_topic_res(&self) -> String {
        self.kafka_topic_res.clone()
    }

    pub fn socket_address(&self) -> SocketAddr {
        self.socket_address
    }

    pub fn token(&self) -> String {
        self.token.clone()
    }
}

impl Default for Config {
    fn default() -> Self {
        let _ = dotenv::dotenv().ok();
        match envy::prefixed(format!("{}_", default_token())).from_env() {
            Err(error) => panic!("{:?}", error),
            Ok(config) => config,
        }
    }
}
