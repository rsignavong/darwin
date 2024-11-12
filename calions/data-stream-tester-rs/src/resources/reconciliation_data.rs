use super::{ProcessorId, ReconciliationDataId};
use data_stream::stream::serialize_ulid;
use serde::Serialize;
use ulid::Ulid;

#[derive(Clone, Serialize, new)]
pub struct ReconciliationData {
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub id: ReconciliationDataId,
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub processor_id: ProcessorId,
    #[new(default)]
    pub path: String,
}

impl ReconciliationData {
    pub fn generate(&mut self, processor_id: ProcessorId) {
        self.id = Ulid::new();
        self.processor_id = processor_id;
        self.path = String::from("/data/tesla/");
    }
}
