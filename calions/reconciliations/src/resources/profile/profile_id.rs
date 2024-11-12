use crate::resources::ResourcesError;
use data_stream::stream::{deserialize_ulid, serialize_ulid};
use derive_more::Display;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use parking_lot::Mutex;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Clone, Debug, Deserialize, Display, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct ProfileId(
    #[serde(
        deserialize_with = "deserialize_ulid",
        serialize_with = "serialize_ulid"
    )]
    Ulid,
);

impl ProfileId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(ProfileId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::ProfileIdGeneration)?,
        ))
    }
}
