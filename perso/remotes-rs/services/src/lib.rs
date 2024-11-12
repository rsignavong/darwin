mod contexts;
mod error;
mod mode;

pub use contexts::*;
pub use error::ServiceError;
pub use mode::Mode;

use repository::{PostgreSql, RepositoryConfig};
use std::env;

#[derive(Clone)]
pub struct Services {
    pub mode: Mode,
    pub postgresql: PostgreSql,
}

impl Services {
    pub fn new(repository: &RepositoryConfig) -> Result<Self, ServiceError> {
        let mode = env::var("RUN_MODE")
            .unwrap_or_else(|_| "development".into())
            .into();
        let postgresql = PostgreSql::new(repository)?;
        Ok(Services { mode, postgresql })
    }
}
