use super::{ActivatedMappingId, ProcessorId, ReconciliationSagaId};
use data_stream::stream::serialize_ulid;
use serde::Serialize;
use ulid::Ulid;

#[derive(Clone, Serialize, new)]
pub struct ReconciliationSaga {
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub id: ReconciliationSagaId,
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub processor_id: ProcessorId,
    #[new(default)]
    #[serde(serialize_with = "serialize_ulid")]
    pub activated_mapping_id: ActivatedMappingId,
}

impl ReconciliationSaga {
    pub fn generate(
        &mut self,
        processor_id: ProcessorId,
        activated_mapping_id: ActivatedMappingId,
    ) {
        self.id = Ulid::new();
        self.processor_id = processor_id;
        self.activated_mapping_id = activated_mapping_id;
    }
}
