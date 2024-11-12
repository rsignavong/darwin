use super::{GraphEdge, GraphEdgePoint};
use super::{GraphNode, GraphNodeChange, GraphNodeChangeList, GraphNodeEvent};
use crate::resources::EntityRecordMetadata;
use crate::resources::EntityRecordRelationshipListByMappingFieldMap;
use crate::resources::NodeEntity;
use crate::resources::{MappingRelationshipMethod, MappingSchemaListByEntityTypeFieldMap};
use ahash::AHashMap;
use derive_new::new;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct GraphMap(#[new(default)] AHashMap<GraphEdge, GraphNode>);

impl GraphMap {
    fn contains_same_edge_point_and_node_id(
        &self,
        edge_point: &Arc<GraphEdgePoint>,
        node_entity: &NodeEntity,
    ) -> Option<&GraphEdge> {
        for (edge, node) in self.0.iter() {
            if node.contains(node_entity.node_id()) && edge_point == edge.point() {
                return Some(edge);
            }
        }

        None
    }

    fn create(
        &mut self,
        edge: &GraphEdge,
        metadata: &EntityRecordMetadata,
        node_entity: &NodeEntity,
    ) -> GraphNodeEvent {
        self.0.insert(
            edge.clone(),
            GraphNode::new(metadata.entity_method().as_ref(), node_entity.node_id()),
        );
        GraphNodeEvent::Created(node_entity.node_id().clone())
    }

    pub fn delete(&mut self, node_entity: &NodeEntity) -> GraphNodeChangeList {
        let mut edges: Vec<GraphEdge> = Vec::new();
        for (edge, node) in self.0.iter() {
            if node.contains(node_entity.node_id()) {
                edges.push(edge.clone());
            }
        }

        let mut changes: Vec<GraphNodeChange> = Vec::new();
        for edge in edges.iter() {
            let event = self.delete_node_and_edge_eventually(edge, node_entity);
            let opposite = self.0.get(&edge.opposite_edge()).map(|n| n.clone());
            changes.push(GraphNodeChange::new(event, opposite));
        }

        GraphNodeChangeList::new(changes)
    }

    fn delete_node_and_edge_eventually(
        &mut self,
        edge: &GraphEdge,
        node_entity: &NodeEntity,
    ) -> GraphNodeEvent {
        if let Some(node) = self.0.get_mut(edge) {
            if node.delete_and_is_empty(node_entity.node_id()) {
                self.0.remove(edge);
            }

            return GraphNodeEvent::Deleted(node_entity.node_id().clone());
        }

        GraphNodeEvent::Ignored(node_entity.node_id().clone())
    }

    pub fn upsert(
        &mut self,
        node_entity: &NodeEntity,
        relationships_by_field: EntityRecordRelationshipListByMappingFieldMap,
        schemas_by_type_field: &MappingSchemaListByEntityTypeFieldMap,
    ) -> GraphNodeChangeList {
        let mut change_list: Vec<Vec<GraphNodeChange>> = Vec::new();

        let metadata_list = relationships_by_field
            .get_entity_record_metadata_list(schemas_by_type_field, node_entity.type_());
        metadata_list.for_each(|metadata| {
            let changes = self.upsert_edges_and_nodes(&metadata, node_entity);
            change_list.push(changes);
        });

        GraphNodeChangeList::new(
            change_list
                .into_iter()
                .flatten()
                .collect::<Vec<GraphNodeChange>>(),
        )
    }

    fn upsert_edges_and_nodes(
        &mut self,
        metadata: &EntityRecordMetadata,
        node_entity: &NodeEntity,
    ) -> Vec<GraphNodeChange> {
        let mut changes: Vec<GraphNodeChange> = Vec::new();
        let edge = metadata.edge();
        let opposite = self.0.get(&edge.opposite_edge()).map(|n| n.clone());

        match metadata.relationship_method().as_ref() {
            MappingRelationshipMethod::Append => {
                if let Some(node) = self.0.get_mut(edge) {
                    let event = node.set(node_entity.node_id().clone());
                    changes.push(GraphNodeChange::new(event, opposite));
                } else {
                    let event = self.create(edge, metadata, node_entity);
                    changes.push(GraphNodeChange::new(event, opposite));
                }
            }
            MappingRelationshipMethod::Fixed => {
                if let Some(edge) =
                    self.contains_same_edge_point_and_node_id(metadata.edge_point(), node_entity)
                {
                    let edge = edge.clone();
                    let opposite = self.0.get(&edge.opposite_edge()).map(|n| n.clone());
                    if let Some(node) = self.0.get_mut(&edge) {
                        let event = node.set(node_entity.node_id().clone());
                        changes.push(GraphNodeChange::new(event, opposite));
                    } else {
                        // this should never happen here
                        let event = self.create(&edge, metadata, node_entity);
                        changes.push(GraphNodeChange::new(event, opposite));
                    }
                } else {
                    let event = self.create(edge, metadata, node_entity);
                    changes.push(GraphNodeChange::new(event, opposite));
                }
            }
            MappingRelationshipMethod::Upsert => {
                if let Some(node) = self.0.get_mut(edge) {
                    let event = node.set(node_entity.node_id().clone());
                    changes.push(GraphNodeChange::new(event, opposite));
                } else {
                    if let Some(edge) = self
                        .contains_same_edge_point_and_node_id(metadata.edge_point(), node_entity)
                    {
                        let edge = edge.clone();
                        let event = self.delete_node_and_edge_eventually(&edge, node_entity);
                        let opposite = self.0.get(&edge.opposite_edge()).map(|n| n.clone());
                        changes.push(GraphNodeChange::new(event, opposite));
                    }

                    let event = self.create(edge, metadata, node_entity);
                    changes.push(GraphNodeChange::new(event, opposite));
                }
            }
        }

        changes
    }
}
