use repository::{DieselError, RepositoryError};
use resources::entities::JobDetailError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum RecruiterJobError {
    #[error("CreateUnknownUser")]
    CreateUnknownUser,
    #[error("JobDetail")]
    JobDetail(#[from] JobDetailError),
    #[error("Diesel")]
    Diesel(#[from] DieselError),
    #[error("Repository")]
    Repository(#[from] RepositoryError),
}
