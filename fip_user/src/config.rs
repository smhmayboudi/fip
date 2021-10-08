use fip_common::constant::{FIP, GROUP, REQ, RES, USER, USER_DB_SQLITE};
use serde::Deserialize;
use std::net::SocketAddr;

/// TODO: documentation
fn default_app_name() -> String {
    env!("CARGO_PKG_NAME").into()
}

/// TODO: documentation
fn default_app_version() -> String {
    env!("CARGO_PKG_VERSION").into()
}

/// TODO: documentation
fn default_database_url() -> String {
    USER_DB_SQLITE.into()
}

/// TODO: documentation
fn default_endpoint() -> String {
    format!("http://{}", default_socket_address())
}

/// TODO: documentation
fn default_kafka_broker() -> String {
    "127.0.0.1:9092".into()
}

/// TODO: documentation
fn default_kafka_consumer_group() -> String {
    format!("{}_{}_{}", FIP, default_token(), GROUP)
}

/// TODO: documentation
fn default_kafka_topic_req() -> String {
    format!("{}_{}_{}", FIP, default_token(), REQ)
}

/// TODO: documentation
fn default_kafka_topic_res() -> String {
    format!("{}_{}_{}", FIP, default_token(), RES)
}

/// TODO: documentation
fn default_socket_address() -> SocketAddr {
    ([0, 0, 0, 0], 8080).into()
}

/// TODO: documentation
fn default_token() -> String {
    USER.into()
}

/// TODO: documentation
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

/// TODO: documentation
impl Config {
    /// TODO: documentation
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

/// TODO: documentation
impl Config {
    /// TODO: documentation
    #[must_use]
    pub fn app_name(&self) -> String {
        self.app_name.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn app_version(&self) -> String {
        self.app_version.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn database_url(&self) -> String {
        self.database_url.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn endpoint(&self) -> String {
        self.endpoint.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn kafka_broker(&self) -> String {
        self.kafka_broker.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn kafka_consumer_group(&self) -> String {
        self.kafka_consumer_group.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn kafka_topic_req(&self) -> String {
        self.kafka_topic_req.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub fn kafka_topic_res(&self) -> String {
        self.kafka_topic_res.clone()
    }

    /// TODO: documentation
    #[must_use]
    pub const fn socket_address(&self) -> SocketAddr {
        self.socket_address
    }

    /// TODO: documentation
    #[must_use]
    pub fn token(&self) -> String {
        self.token.clone()
    }
}

/// TODO: documentation
impl Default for Config {
    /// TODO: documentation
    fn default() -> Self {
        let _path = dotenv::dotenv().ok();
        match envy::prefixed(format!("{}_", default_token())).from_env() {
            Err(err) => panic!("{:?}", err),
            Ok(config) => config,
        }
    }
}
