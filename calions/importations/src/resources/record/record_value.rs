use crate::decoders::Mapping;
use crate::resources::ResourcesError;
use csv::StringRecord;
use derive_more::FromStr;
use serde::Serialize;

#[derive(Debug, FromStr, Serialize)]
pub struct RecordValue(String);

impl RecordValue {
    pub fn from_mapping(record: &StringRecord, mapping: &Mapping) -> Result<Self, ResourcesError> {
        record
            .get(mapping.column.to_usize())
            .ok_or(ResourcesError::RecordValueNotFound(mapping.column.clone()))?
            .parse()
            .map_err(|source| ResourcesError::RecordValueFromStr { source })
    }
}
