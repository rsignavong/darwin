use super::GdprError;
use data_stream::stream::{deserialize_ulid, serialize_ulid};
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use ulid::{Generator, Ulid};
use once_cell::sync::Lazy;
use parking_lot::Mutex;

static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Clone, Debug, Deref, Deserialize, Display, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct GdprKeyId(
    #[serde(
        deserialize_with = "deserialize_ulid",
        serialize_with = "serialize_ulid"
    )]
    Ulid,
);

impl TryFrom<&str> for GdprKeyId {
    type Error = GdprError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ulid = Ulid::from_string(s).map_err(|source| GdprError::GdprKeyIdTryFromStr(source))?;

        Ok(GdprKeyId(ulid))
    }
}

impl GdprKeyId {
    pub fn new() -> Result<Self, GdprError> {
        Ok(GdprKeyId(
            GENERATOR.lock()
                .generate()
                .map_err(GdprError::GdprKeyIdGeneration)?,
        ))
    }
}
