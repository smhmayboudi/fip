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

    Trace::init(&config)?;

    Server::init(&config).await
}
