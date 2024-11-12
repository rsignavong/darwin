use crate::resources::{MappingMatchingId, RecordValue, ResourcesError};
use derive_more::Display;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize, new)]
#[display(fmt = "{}:{}", _0, _1)]
#[serde(into = "String", try_from = "String")]
pub struct ReconciliationKey(Arc<MappingMatchingId>, Arc<RecordValue>);

impl Into<String> for ReconciliationKey {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TryFrom<String> for ReconciliationKey {
    type Error = ResourcesError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut v = s.split(":");
        if v.clone().count() != 2 {
            return Err(ResourcesError::ReconciliationKeyInvalid);
        }
        let mapping_matching_id =
            Arc::new(MappingMatchingId::try_from(v.next().ok_or_else(|| {
                ResourcesError::ReconciliationKeyDeserializeMappingMatchingId
            })?)?);
        let record_value = Arc::new(RecordValue::new(
            v.next()
                .ok_or_else(|| ResourcesError::ReconciliationKeyDeserializeRecordValue)?
                .to_owned(),
        ));

        Ok(ReconciliationKey(mapping_matching_id, record_value))
    }
}
