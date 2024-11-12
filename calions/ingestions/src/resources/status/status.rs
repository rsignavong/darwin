use super::{StatusIngestion, StatusState};
use crate::resources::{ContactCount, MappingId, OrganizationId, ProcessorId};
use derive_new::new;
use serde::Serialize;
use std::sync::Arc;

#[derive(Clone, Debug, Serialize, new)]
pub struct Status {
    pub organization_id: Arc<OrganizationId>,
    pub processor_id: Arc<ProcessorId>,
    #[new(default)]
    #[serde(rename = "activated_mapping_id")]
    pub mapping_id: Option<Arc<MappingId>>,
    #[new(default)]
    pub contacts_count: Arc<ContactCount>,
    #[new(default)]
    pub ingestions: StatusIngestion,
    #[new(default)]
    pub state: StatusState,
}
