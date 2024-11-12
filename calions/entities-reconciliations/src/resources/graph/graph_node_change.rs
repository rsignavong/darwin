use super::{GraphNode, GraphNodeEvent};
use crate::resources::NodeId;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct GraphNodeChange {
    event: GraphNodeEvent,
    opposite: Option<GraphNode>,
}

impl GraphNodeChange {
    pub fn created_event(&self) -> Option<&Arc<NodeId>> {
        self.event.created()
    }

    pub fn deleted_event(&self) -> Option<&Arc<NodeId>> {
        self.event.deleted()
    }

    pub fn opposite(&self) -> Option<&GraphNode> {
        self.opposite.as_ref()
    }
}
