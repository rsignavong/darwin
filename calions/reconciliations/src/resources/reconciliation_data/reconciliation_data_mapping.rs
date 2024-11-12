use crate::resources::{MappingField, MappingMatchingId};
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize, new)]
pub struct ReconciliationDataMapping {
    pub field: Arc<MappingField>,
    pub mapping_matching_id: Arc<MappingMatchingId>,
}
