use super::{NodeAdjacencyListMap, NodeAdjacencySet, NodeEntity, NodeEntityMaps, NodeId};
use crate::resources::ResourcesError;
use ahash::AHashSet;
use itertools::Itertools;
use std::sync::Arc;

#[derive(Debug)]
pub struct NodeConnectedSet(AHashSet<Arc<NodeId>>);

impl NodeConnectedSet {
    fn connect_nodes(
        &mut self,
        node_id: &Arc<NodeId>,
        adjacency_list: &NodeAdjacencyListMap,
    ) -> Option<()> {
        let adjacency_set = adjacency_list.get_adjacency_set(node_id)?;
        let diffs = adjacency_set.diff(self.unwrap());
        self.union(&adjacency_set);
        for diff in diffs.iter() {
            self.connect_nodes(diff, adjacency_list)?;
        }

        Some(())
    }

    pub fn map_entity_and_group_by_type(
        self,
        nodes: &NodeEntityMaps,
    ) -> Result<Vec<Vec<NodeEntity>>, ResourcesError> {
        let mut node_entities = self
            .0
            .into_iter()
            .map(|n| {
                nodes
                    .get_node_entity(&n)
                    .ok_or_else(|| ResourcesError::NodesConnectedListMapEntityNotFound(n))
            })
            .collect::<Result<Vec<NodeEntity>, ResourcesError>>()?;

        node_entities.sort_unstable();

        let node_entities_group = node_entities
            .into_iter()
            .group_by(|ne| ne.get_type().clone())
            .into_iter()
            .map(|(_, node_entities)| node_entities.collect::<Vec<_>>())
            .collect();

        Ok(node_entities_group)
    }

    pub fn new(
        node_id: &Arc<NodeId>,
        adjacency_list: &NodeAdjacencyListMap,
    ) -> Result<Self, ResourcesError> {
        let mut connected_list = NodeConnectedSet(AHashSet::new());
        connected_list
            .connect_nodes(node_id, adjacency_list)
            .ok_or_else(|| ResourcesError::NodeConnectedSet)?;
        Ok(connected_list)
    }

    pub fn union(&mut self, other_node_ids: &NodeAdjacencySet) {
        self.0 = self
            .0
            .union(other_node_ids.unwrap())
            .map(|n| n.clone())
            .collect();
    }

    pub fn unwrap(&self) -> &AHashSet<Arc<NodeId>> {
        &self.0
    }
}
