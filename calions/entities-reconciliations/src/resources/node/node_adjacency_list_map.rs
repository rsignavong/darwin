use super::{NodeAdjacencySet, NodeChange, NodeId};
use ahash::AHashMap;
use derive_new::new;
use std::sync::Arc;

#[derive(Debug, new)]
pub struct NodeAdjacencyListMap(#[new(default)] AHashMap<Arc<NodeId>, NodeAdjacencySet>);

impl NodeAdjacencyListMap {
    pub fn add(&mut self, change: &NodeChange) {
        let node_id = change.event_node_id();
        let opposite = change.opposite_node_id();
        if let Some(set) = self.0.get_mut(node_id) {
            set.add(opposite.clone());
        } else {
            self.0
                .insert(node_id.clone(), NodeAdjacencySet::new(opposite.clone()));
        }

        if let Some(set) = self.0.get_mut(opposite) {
            set.add(node_id.clone());
        } else {
            self.0
                .insert(opposite.clone(), NodeAdjacencySet::new(node_id.clone()));
        }
    }

    pub fn get_adjacency_set(&self, node_id: &Arc<NodeId>) -> Option<&NodeAdjacencySet> {
        self.0.get(node_id)
    }

    pub fn remove(&mut self, change: &NodeChange) {
        let node_id = change.event_node_id();
        let opposite = change.opposite_node_id();

        let node_id_set_is_empty = if let Some(set) = self.0.get_mut(node_id) {
            set.remove(opposite)
        } else {
            false
        };

        let opposite_set_is_empty = if let Some(set) = self.0.get_mut(opposite) {
            set.remove(node_id)
        } else {
            false
        };

        if node_id_set_is_empty {
            self.0.remove(node_id);
        }

        if opposite_set_is_empty {
            self.0.remove(node_id);
        }
    }
}
