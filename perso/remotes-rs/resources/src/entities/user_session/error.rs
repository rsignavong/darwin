use thiserror::Error;
use ulid::MonotonicError as UlidError;

#[derive(Debug, Error)]
pub enum UserSessionError {
    #[error("InvalidCode {0}")]
    InvalidCode(String),
    #[error("TokenGeneration {0}")]
    TokenGeneration(UlidError),
}
