use super::ReconciliationData;
use crate::resources::ProcessorId;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct ReconciliationDataRecord {
    pub data: ReconciliationData,
    pub processor_id: Arc<ProcessorId>,
}
