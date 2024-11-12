mod application;
mod cli;

use anyhow::{Context, Result};
use application::{App, Config};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    info!("Configuration: {:?}", &Config::get());

    App::start().with_context(|| "Allium Starter")?;

    Ok(())
}
