use super::EntityRecordRelationship;
use crate::resources::MappingSchema;
use crate::resources::{GraphEdge, GraphEdgePoint};
use crate::resources::{MappingEntityMethod, MappingRelationshipMethod, MappingSchemaMethods};
use std::sync::Arc;

pub struct EntityRecordMetadata {
    edge: GraphEdge,
    methods: MappingSchemaMethods,
}

impl EntityRecordMetadata {
    pub fn edge(&self) -> &GraphEdge {
        &self.edge
    }

    pub fn edge_point(&self) -> &Arc<GraphEdgePoint> {
        self.edge.point()
    }

    pub fn entity_method(&self) -> &Arc<MappingEntityMethod> {
        self.methods.entity()
    }

    pub fn new(relationship: Arc<EntityRecordRelationship>, schema: &MappingSchema) -> Self {
        let edge = GraphEdge::new(schema.edge_point().clone(), relationship.clone());

        EntityRecordMetadata {
            edge,
            methods: schema.methods().clone(),
        }
    }

    pub fn relationship_method(&self) -> &Arc<MappingRelationshipMethod> {
        self.methods.relationship()
    }
}
