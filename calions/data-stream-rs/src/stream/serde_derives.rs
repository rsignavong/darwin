use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use std::collections::HashMap;
use std::hash::Hash;
use std::str::FromStr;
use ulid::Ulid;

pub fn deserialize_hashmap<'de, D, K, B>(d: D) -> Result<HashMap<K, B>, D::Error>
where
    D: Deserializer<'de>,
    K: FromStr + Eq + Hash,
    B: Deserialize<'de>,
{
    fn deserialize_string_key<'de, D, S>(d: D) -> std::result::Result<S, D::Error>
    where
        D: Deserializer<'de>,
        S: FromStr,
    {
        let s: String = Deserialize::deserialize(d).map_err(serde::de::Error::custom)?;
        s.parse::<S>()
            .map_err(|_| serde::de::Error::custom(format!("Invalid key: {}", s)))
    }

    #[derive(Deserialize, Hash, Eq, PartialEq)]
    struct Wrapper<S: FromStr>(#[serde(deserialize_with = "deserialize_string_key")] S);
    #[allow(clippy::mutable_key_type)]
    let dict: HashMap<Wrapper<K>, B> = Deserialize::deserialize(d)?;
    Ok(dict.into_iter().map(|(Wrapper(k), v)| (k, v)).collect())
}

pub fn deserialize_object<'de, D, M>(d: D) -> Result<Option<M>, D::Error>
where
    D: Deserializer<'de>,
    M: Deserialize<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    let v: Value = serde_json::from_str(s).map_err(serde::de::Error::custom)?;
    match v {
        Value::Object(m) => {
            if m.is_empty() {
                Ok(None)
            } else {
                let obj = serde_json::from_str::<M>(s).map_err(serde::de::Error::custom)?;
                Ok(Some(obj))
            }
        }
        _ => Err(serde::de::Error::custom("expected an object {}")),
    }
}

pub fn deserialize_ulid<'de, D>(d: D) -> Result<Ulid, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    Ulid::from_string(s).map_err(serde::de::Error::custom)
}

pub fn serialize_object<S, M>(obj: &Option<M>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    M: Serialize,
{
    #[derive(Serialize)]
    struct EmptyObject {}

    match obj {
        Some(o) => o.serialize(s),
        None => {
            let empty = EmptyObject {};
            empty.serialize(s)
        }
    }
}

pub fn serialize_ulid<S>(ulid: &Ulid, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(&ulid.to_string())
}
