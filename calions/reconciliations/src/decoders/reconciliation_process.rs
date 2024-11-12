use crate::resources::OrganizationId;
use crate::resources::{MappingContactPoint, MappingField, MappingId};
use crate::resources::{ProcessorId, ProcessorTopic};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationProcessMetadata;

#[derive(Clone, Debug, Deserialize)]
pub struct ActivatedMapping {
    pub field_alias: Arc<MappingField>,
    pub is_contact_point: MappingContactPoint,
    pub matching_mappings: Vec<Arc<MappingField>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ActivatedMappings {
    pub id: Arc<MappingId>,
    pub mappings: Arc<Vec<ActivatedMapping>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationProcessBody {
    pub activated_mappings: ActivatedMappings,
    #[serde(alias = "gdpr_data_anonymization_request_validations_topics")]
    pub anonymizations_topics: Arc<Vec<ProcessorTopic>>,
    pub organization_id: Arc<OrganizationId>,
    pub processor_id: Arc<ProcessorId>,
    pub raw_records_topics: Arc<Vec<ProcessorTopic>>,
    pub reconciliations_records_topic: Arc<ProcessorTopic>,
}
