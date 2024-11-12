use crate::resources::{IngestionData, ProcessorId};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct IngestionDataMetadata {
    pub processor_id: Arc<ProcessorId>,
}

pub type IngestionDataBody = IngestionData;
