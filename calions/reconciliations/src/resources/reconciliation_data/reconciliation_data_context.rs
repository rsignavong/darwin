use crate::resources::{MappingId, OrganizationId};
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize, new)]
pub struct ReconciliationDataContext {
    pub mapping_id: Arc<MappingId>,
    pub organization_id: Arc<OrganizationId>,
}
