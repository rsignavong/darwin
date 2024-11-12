mod applications;

use anyhow::{Context, Result};
use application::{App, Config};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    App::start().with_context(|| "Flex Office Jobs")?;

    Ok(())
}
