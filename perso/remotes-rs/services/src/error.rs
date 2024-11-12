use crate::contexts::{ListingError, RecruiterJobError, StatusError};
use actix_web::ResponseError;
use repository::RepositoryError;
use resources::ResourceError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Listing")]
    Listing(#[from] ListingError),
    #[error("RecruiterJob")]
    RecruiterJob(#[from] RecruiterJobError),
    #[error("Status")]
    Status(#[from] StatusError),
    #[error("Repository")]
    Repository(#[from] RepositoryError),
    #[error("Resource")]
    Resource(#[from] ResourceError),
}

// impl ResponseError for ServiceError {}
