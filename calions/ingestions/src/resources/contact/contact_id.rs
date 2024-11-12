use super::ContactError;
use data_stream::stream::{deserialize_ulid, serialize_ulid};
use derive_more::{Deref, Display};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use ulid::Ulid;

#[derive(Clone, Debug, Deref, Deserialize, Display, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct ContactId(
    #[serde(
        deserialize_with = "deserialize_ulid",
        serialize_with = "serialize_ulid"
    )]
    Ulid,
);

impl TryFrom<&str> for ContactId {
    type Error = ContactError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let ulid =
            Ulid::from_string(s).map_err(|source| ContactError::ContactIdTryFromStr(source))?;

        Ok(ContactId(ulid))
    }
}
