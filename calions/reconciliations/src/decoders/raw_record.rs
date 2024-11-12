use crate::resources::{MappingId, RecordData, RecordSource};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct RawRecordMetadata {
    pub activated_mapping_id: Arc<MappingId>,
    pub source: Arc<RecordSource>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RawRecordBody {
    pub record: Arc<RecordData>,
}
