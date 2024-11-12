use crate::resources::ResourcesError;
use serde::Serialize;
use serde_json::Value;
use std::convert::TryFrom;

#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct EntityRecordId(String);

impl TryFrom<&Value> for EntityRecordId {
    type Error = ResourcesError;

    fn try_from(v: &Value) -> Result<Self, Self::Error> {
        if !v.is_string() {
            return Err(ResourcesError::EntityRecordId);
        }

        let s = v.as_str().ok_or_else(|| ResourcesError::EntityRecordId)?;

        Ok(EntityRecordId(s.into()))
    }
}
