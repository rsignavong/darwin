use crate::resources::ResourcesError;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct NodeId(Ulid);

impl NodeId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(NodeId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::NodeId)?,
        ))
    }
}
