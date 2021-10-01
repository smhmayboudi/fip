pub mod config;
pub mod controller;
pub mod model;
pub mod proto;
pub mod repository;
pub mod server;
pub mod service;
pub mod trace;

use crate::{config::Config, server::Server, trace::Trace};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();

    Trace::init(&config)?;

    Server::init(config.clone()).await
}
