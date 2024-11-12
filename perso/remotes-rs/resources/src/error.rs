use crate::entities::EntityError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ResourceError {
    #[error("Entity")]
    Entity(#[from] EntityError),
}
