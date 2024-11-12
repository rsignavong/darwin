use serde::{Deserialize, Deserializer, Serializer};
use ulid::Ulid;

pub fn deserialize_ulid<'de, D>(d: D) -> Result<Ulid, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    Ulid::from_string(s).map_err(serde::de::Error::custom)
}

pub fn serialize_ulid<S>(ulid: &Ulid, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&ulid.to_string())
}
