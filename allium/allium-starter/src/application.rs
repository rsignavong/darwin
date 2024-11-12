#[path = "application/config.rs"]
mod config;
mod error;

pub use self::config::Config;
pub use error::ApplicationError;

use crate::cli::Cli;
use clap::Parser;

pub struct App;

impl App {
    pub fn start() -> Result<(), ApplicationError> {
        let cli = Cli::parse();
        cli.execute()?;

        Ok(())
    }
}
