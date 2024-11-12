use crate::resources::{ProcessorId, ReconciliationData};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationDataMetadata {
    pub processor_id: Arc<ProcessorId>,
}

pub type ReconciliationDataBody = ReconciliationData;
