use repository::{DieselError, RepositoryError};
use resources::entities::JobListingError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ListingError {
    #[error("JobListing")]
    JobListing(#[from] JobListingError),
    #[error("Diesel")]
    Diesel(#[from] DieselError),
    #[error("Repository")]
    Repository(#[from] RepositoryError),
}
