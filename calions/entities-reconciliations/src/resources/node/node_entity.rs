use super::NodeId;
use crate::resources::{Entity, EntityType};
use derive_new::new;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, Ord, PartialOrd, PartialEq, new)]
pub struct NodeEntity {
    entity: Arc<Entity>,
    node_id: Arc<NodeId>,
}

impl NodeEntity {
    pub fn entity(&self) -> &Arc<Entity> {
        &self.entity
    }

    pub fn get_type(&self) -> &EntityType {
        self.entity.get_type()
    }

    pub fn is_same_type(&self, type_: &EntityType) -> bool {
        self.entity.is_same_type(type_)
    }

    pub fn node_id(&self) -> &Arc<NodeId> {
        &self.node_id
    }

    pub fn type_(&self) -> &Arc<EntityType> {
        self.entity.type_()
    }
}
