use super::key::GdprKey;
use super::value::GdprValue;
use serde::{Deserialize, Deserializer, Serializer};
use std::convert::TryFrom;

pub fn deserialize_gdpr_key<'de, D>(d: D) -> Result<GdprKey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    GdprKey::try_from(s).map_err(serde::de::Error::custom)
}

pub fn serialize_gdpr_key<S>(key: &GdprKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&key.to_string())
}

pub fn deserialize_gdpr_key_from_bytes<'de, D>(d: D) -> Result<GdprKey, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &[u8] = Deserialize::deserialize(d)?;
    GdprKey::try_from(s).map_err(serde::de::Error::custom)
}

pub fn serialize_gdpr_key_as_bytes<S>(key: &GdprKey, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_bytes(&key.as_bytes())
}

pub fn deserialize_gdpr_value<'de, D>(d: D) -> Result<GdprValue, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    GdprValue::try_from(s).map_err(serde::de::Error::custom)
}

pub fn serialize_gdpr_value<S>(value: &GdprValue, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&value.to_string())
}
