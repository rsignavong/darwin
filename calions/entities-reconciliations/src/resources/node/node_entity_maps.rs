use super::{NodeEntity, NodeId};
use crate::resources::{Entity, EntityRecordId, EntityType, ResourcesError};
use ahash::AHashMap;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct NodeEntityMaps {
    #[new(default)]
    nodes: AHashMap<Arc<NodeId>, Arc<Entity>>,
    #[new(default)]
    entities: AHashMap<Arc<Entity>, Arc<NodeId>>,
}

impl NodeEntityMaps {
    fn add_entity(&mut self, entity: &Arc<Entity>) -> Result<Arc<NodeId>, ResourcesError> {
        let node_id = Arc::new(NodeId::new()?);
        self.nodes.insert(node_id.clone(), entity.clone());
        self.entities.insert(entity.clone(), node_id.clone());

        Ok(node_id)
    }

    pub fn delete(&mut self, node_entity: NodeEntity) {
        self.nodes.remove(node_entity.node_id());
        self.entities.remove(node_entity.entity());
    }

    pub fn get_node_entity(&self, node_id: &Arc<NodeId>) -> Option<NodeEntity> {
        self.nodes
            .get(node_id)
            .map(|e| NodeEntity::new(e.clone(), node_id.clone()))
    }

    pub fn lookup(&self, record_id: EntityRecordId, type_: &Arc<EntityType>) -> Option<NodeEntity> {
        let entity = Arc::new(Entity::new(type_.clone(), record_id));
        self.entities
            .get(&entity)
            .map(|node_id| NodeEntity::new(entity, node_id.clone()))
    }

    pub fn lookup_or_create(
        &mut self,
        record_id: EntityRecordId,
        type_: &Arc<EntityType>,
    ) -> Result<NodeEntity, ResourcesError> {
        let entity = Arc::new(Entity::new(type_.clone(), record_id));
        let node_id = if let Some(node_id) = self.entities.get(&entity) {
            node_id.clone()
        } else {
            self.add_entity(&entity)?
        };

        Ok(NodeEntity::new(entity, node_id))
    }
}
