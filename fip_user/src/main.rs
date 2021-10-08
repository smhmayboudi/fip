//! TODO: documentation

/// TODO: documentation
pub mod config;
/// TODO: documentation
pub mod controller;
/// TODO: documentation
pub mod model;
/// TODO: documentation
pub mod proto;
/// TODO: documentation
pub mod repository;
/// TODO: documentation
pub mod server;
/// TODO: documentation
pub mod service;
/// TODO: documentation
pub mod trace;

use crate::{config::Config, server::Server, trace::Trace};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::new();

    // let file_appender = tracing_appender::rolling::daily("log", "prefix.log");
    // let (non_blocking_writer, _guard) = tracing_appender::non_blocking(file_appender);
    let (non_blocking_writer, _guard) = tracing_appender::non_blocking(std::io::stdout());
    Trace::init(&config, non_blocking_writer)?;

    Server::init(&config).await
}
