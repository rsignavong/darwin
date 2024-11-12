use derive_more::Display;
use derive_new::new;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(
    Clone, Debug, Deserialize, Display, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, new,
)]
pub struct RecordValue(#[serde(deserialize_with = "deserialize_record")] String);

impl RecordValue {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

pub fn deserialize_record<'de, D>(d: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    Ok(s.trim().to_owned())
}
