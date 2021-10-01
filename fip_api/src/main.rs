pub mod api;
pub mod at;
pub mod auth;
pub mod common;
pub mod config;
pub mod jwks;
pub mod rt;
pub mod server;
pub mod trace;
pub mod user;

use crate::{config::Config, server::Server, trace::Trace};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::default();

    Trace::init(&config)?;

    Server::init(config.clone()).await
}
