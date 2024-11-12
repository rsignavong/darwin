use crate::resources::EntityType;
use crate::resources::{MappingEntityMethod, MappingField, MappingId, MappingRelationshipMethod};
use crate::resources::{ProcessorId, ProcessorTopic};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationCommandMetadata;

#[derive(Clone, Debug, Deserialize)]
pub struct Entity {
    pub topics: Vec<Arc<ProcessorTopic>>,
    #[serde(rename = "type")]
    pub type_: Arc<EntityType>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RelationshipEntity {
    #[serde(rename = "type")]
    pub type_: Arc<EntityType>,
    pub matching_mappings: Vec<Arc<MappingField>>,
    pub method: Arc<MappingEntityMethod>,
    pub relationship_method: Arc<MappingRelationshipMethod>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Relationship {
    pub entity: RelationshipEntity,
    pub belongs_to: RelationshipEntity,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mappings {
    pub entities: Vec<Arc<Entity>>,
    pub relationships: Vec<Relationship>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reconciliations {
    pub entities: Vec<Arc<EntityType>>,
    pub topic: Arc<ProcessorTopic>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationCommandBody {
    pub activated_mapping_id: Arc<MappingId>,
    pub mappings: Mappings,
    pub processor_id: Arc<ProcessorId>,
    pub reconciliations: Reconciliations,
}
