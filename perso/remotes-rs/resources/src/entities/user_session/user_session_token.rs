use super::UserSessionError;
use derive_more::Display;
#[cfg(feature = "backend")]
use once_cell::sync::Lazy;
#[cfg(feature = "backend")]
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use ulid::{Generator, Ulid};
use utils::serializers::{deserialize_ulid, serialize_ulid};

#[cfg(feature = "backend")]
static GENERATOR: Lazy<Mutex<Generator>> = Lazy::new(|| Mutex::new(Generator::new()));

#[derive(Debug, Deserialize, Display, Eq, PartialEq, Serialize)]
pub struct UserSessionToken(
    #[serde(
        deserialize_with = "deserialize_ulid",
        serialize_with = "serialize_ulid"
    )]
    Ulid,
);

impl UserSessionToken {
    #[cfg(feature = "backend")]
    pub fn new() -> Result<Self, UserSessionError> {
        Ok(UserSessionToken(
            GENERATOR
                .lock()
                .generate()
                .map_err(UserSessionError::TokenGeneration)?,
        ))
    }
}
