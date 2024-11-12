use crate::resources::{MappingAppendable, MappingField, MappingGdpr, MappingId, OrganizationId};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct ActivatedMappingMetadata;

#[derive(Clone, Debug, Deserialize)]
pub struct ActivatedMapping {
    pub field_alias: Arc<MappingField>,
    pub is_appendable: MappingAppendable,
    pub is_gdpr: MappingGdpr,
    pub matching_mappings: Vec<Arc<MappingField>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActivatedMappingBody {
    pub id: Arc<MappingId>,
    pub mappings: Arc<Vec<ActivatedMapping>>,
    pub organization_id: Arc<OrganizationId>,
}
