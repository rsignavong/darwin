use super::GraphEdgePoint;
use crate::resources::EntityRecordRelationship;
use derive_new::new;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, PartialEq, new)]
pub struct GraphEdge {
    point: Arc<GraphEdgePoint>,
    relationship: Arc<EntityRecordRelationship>,
}

impl GraphEdge {
    pub fn point(&self) -> &Arc<GraphEdgePoint> {
        &self.point
    }

    pub fn opposite_edge(&self) -> Self {
        GraphEdge {
            point: Arc::new(self.point.opposite()),
            relationship: self.relationship.clone(),
        }
    }
}
