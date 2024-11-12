use crate::resources::ResourcesError;
use data_stream::stream::serialize_ulid;
use derive_more::Deref;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::Serialize;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Debug, Deref, Serialize)]
pub struct RecordId(#[serde(serialize_with = "serialize_ulid")] Ulid);

impl RecordId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(RecordId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::RecordIdGeneration)?,
        ))
    }
}
