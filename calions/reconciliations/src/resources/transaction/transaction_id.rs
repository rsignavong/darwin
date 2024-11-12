use crate::resources::ResourcesError;
use data_stream::stream::serialize_ulid;
use derive_more::Display;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::Serialize;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Clone, Debug, Display, Serialize)]
pub struct TransactionId(#[serde(serialize_with = "serialize_ulid")] Ulid);

impl TransactionId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(TransactionId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::TransactionIdGeneration)?,
        ))
    }
}
