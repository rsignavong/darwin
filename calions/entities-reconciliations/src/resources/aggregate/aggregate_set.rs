use crate::resources::NodeId;
use std::sync::Arc;

#[derive(Debug, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AggregateSet(Vec<Arc<NodeId>>);

impl AggregateSet {
    pub fn add_node_id(&mut self, node_id: &Arc<NodeId>) {
        self.0.push(node_id.clone());
    }

    pub fn with_capacity(capacity: usize) -> Self {
        AggregateSet(Vec::with_capacity(capacity))
    }
}
