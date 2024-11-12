use crate::cli::CliError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Command line")]
    Cli(#[from] CliError),
}
