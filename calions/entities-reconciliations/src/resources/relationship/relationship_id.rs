use crate::resources::ResourcesError;
use derive_more::Display;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RelationshipId(Ulid);

impl RelationshipId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(RelationshipId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::RelationshipId)?,
        ))
    }
}
