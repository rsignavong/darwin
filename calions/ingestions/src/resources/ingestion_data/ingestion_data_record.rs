use super::IngestionData;
use crate::resources::ProcessorId;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct IngestionDataRecord {
    pub data: IngestionData,
    pub processor_id: Arc<ProcessorId>,
}
