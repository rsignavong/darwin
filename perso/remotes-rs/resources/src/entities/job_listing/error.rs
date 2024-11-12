use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Debug, Error)]
pub enum JobListingError {
    #[error("InvalidUuid")]
    InvalidUuid(#[from] UuidError),
}
