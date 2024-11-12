use super::{StatusReconciliation, StatusState};
use crate::resources::{MappingId, ProcessorId, ReconciliationCount};
use derive_new::new;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, new)]
pub struct Status {
    pub processor_id: Arc<ProcessorId>,
    #[new(default)]
    #[serde(rename = "activated_mapping_id")]
    pub mapping_id: Option<Arc<MappingId>>,
    #[new(default)]
    pub reconciliations: StatusReconciliation,
    #[new(default)]
    pub reconciliations_count: ReconciliationCount,
    #[new(default)]
    pub state: StatusState,
}
