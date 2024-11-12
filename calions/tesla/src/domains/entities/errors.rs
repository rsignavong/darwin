use crate::core::TeslaError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum EntityError {
    Tesla(TeslaError),
}

impl Display for EntityError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            EntityError::Tesla(ref e) => write!(f, "Entity::Tesla: {}", e),
        }
    }
}
impl From<TeslaError> for EntityError {
    fn from(err: TeslaError) -> Self {
        EntityError::Tesla(err)
    }
}

