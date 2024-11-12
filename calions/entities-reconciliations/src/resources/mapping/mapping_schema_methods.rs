use super::{MappingEntityMethod, MappingRelationshipMethod};
use derive_new::new;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq, new)]
pub struct MappingSchemaMethods {
    entity: Arc<MappingEntityMethod>,
    relationship: Arc<MappingRelationshipMethod>,
}

impl MappingSchemaMethods {
    pub fn entity(&self) -> &Arc<MappingEntityMethod> {
        &self.entity
    }

    pub fn relationship(&self) -> &Arc<MappingRelationshipMethod> {
        &self.relationship
    }
}
