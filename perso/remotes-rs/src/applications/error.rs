use repository::RepositoryError;
use services::ServiceError;
use thiserror::Error;
use web::WebError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Repository")]
    Repository(#[from] RepositoryError),
    #[error("Service")]
    Service(#[from] ServiceError),
    #[error("Web")]
    Web(#[from] WebError),
}
