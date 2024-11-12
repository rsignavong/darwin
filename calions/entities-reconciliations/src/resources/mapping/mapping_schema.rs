use super::MappingSchemaMethods;
use crate::resources::GraphEdgePoint;
use derive_new::new;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, PartialEq, new)]
pub struct MappingSchema {
    edge_point: Arc<GraphEdgePoint>,
    methods: MappingSchemaMethods,
}

impl MappingSchema {
    pub fn edge_point(&self) -> &Arc<GraphEdgePoint> {
        &self.edge_point
    }

    pub fn methods(&self) -> &MappingSchemaMethods {
        &self.methods
    }
}
