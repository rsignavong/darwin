use crate::resources::ResourcesError;
use data_stream::stream::{deserialize_ulid, serialize_ulid};
use derive_more::Display;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use ulid::{Generator, Ulid};

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Debug, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct MappingMatchingId(
    #[serde(
        deserialize_with = "deserialize_ulid",
        serialize_with = "serialize_ulid"
    )]
    Ulid,
);

impl TryFrom<&str> for MappingMatchingId {
    type Error = ResourcesError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ulid = Ulid::from_string(s).map_err(ResourcesError::MappingMatchingTryFromStr)?;
        Ok(MappingMatchingId(ulid))
    }
}

impl MappingMatchingId {
    pub fn new() -> Result<Self, ResourcesError> {
        Ok(MappingMatchingId(
            GENERATOR
                .lock()
                .generate()
                .map_err(ResourcesError::MappingMatchingId)?,
        ))
    }
}
