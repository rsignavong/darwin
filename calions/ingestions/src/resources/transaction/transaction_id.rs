use data_stream::stream::{deserialize_ulid, serialize_ulid};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Clone, Debug, Deserialize, Display, Eq, PartialEq, PartialOrd, Ord, Serialize)]
pub struct TransactionId(
    #[serde(
        deserialize_with = "deserialize_ulid",
        serialize_with = "serialize_ulid"
    )]
    Ulid,
);
