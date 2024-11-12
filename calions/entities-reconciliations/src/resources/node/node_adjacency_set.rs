use super::NodeId;
use ahash::AHashSet;
use std::sync::Arc;

#[derive(Debug)]
pub struct NodeAdjacencySet(AHashSet<Arc<NodeId>>);

impl NodeAdjacencySet {
    pub fn add(&mut self, node_id: Arc<NodeId>) {
        self.0.insert(node_id);
    }

    pub fn diff(&self, connected_list: &AHashSet<Arc<NodeId>>) -> AHashSet<Arc<NodeId>> {
        self.0
            .difference(connected_list)
            .map(|n| n.clone())
            .collect()
    }

    pub fn new(node_id: Arc<NodeId>) -> Self {
        let mut set: AHashSet<Arc<NodeId>> = AHashSet::new();
        set.insert(node_id);
        NodeAdjacencySet(set)
    }

    pub fn remove(&mut self, node_id: &Arc<NodeId>) -> bool {
        self.0.remove(node_id);
        self.0.is_empty()
    }

    pub fn unwrap(&self) -> &AHashSet<Arc<NodeId>> {
        &self.0
    }
}
