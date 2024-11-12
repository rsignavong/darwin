use super::NodeId;
use ahash::AHashSet;
use derive_new::new;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct NodeChange {
    event: Arc<NodeId>,
    opposite: Arc<NodeId>,
}

impl NodeChange {
    pub fn event_node_id(&self) -> &Arc<NodeId> {
        &self.event
    }

    pub fn opposite_node_id(&self) -> &Arc<NodeId> {
        &self.opposite
    }

    pub fn tuple_set(&self) -> AHashSet<Arc<NodeId>> {
        let mut set: AHashSet<Arc<NodeId>> = AHashSet::new();
        set.insert(self.event.clone());
        set.insert(self.opposite.clone());
        set
    }
}
