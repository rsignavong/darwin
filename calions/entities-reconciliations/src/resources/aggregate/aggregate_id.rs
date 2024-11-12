use crate::resources::ResourcesError;
use data_stream::stream::serialize_ulid;
use derive_more::Display;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::Serialize;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Clone, Debug, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct AggregateId(#[serde(serialize_with = "serialize_ulid")] Ulid);

impl AggregateId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(AggregateId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::AggregateId)?,
        ))
    }
}
