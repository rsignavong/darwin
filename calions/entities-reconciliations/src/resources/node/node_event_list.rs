use super::{NodeAdjacencyListMap, NodeChanges, NodeConnectedSet, NodeEntityMaps, NodeId};
use crate::resources::{AggregateEventList, AggregateMap, ReconciliationOrder, ResourcesError};
use std::sync::Arc;

#[derive(Debug)]
pub enum NodeEventList {
    Created(Vec<Arc<NodeId>>),
    Deleted(Vec<Arc<NodeId>>),
}

impl NodeEventList {
    pub fn aggregate(
        &self,
        aggregates: &mut AggregateMap,
        adjacency: &mut NodeAdjacencyListMap,
        node_changes: &NodeChanges,
        node_entities: &NodeEntityMaps,
        order: &ReconciliationOrder,
    ) -> Result<AggregateEventList, ResourcesError> {
        let mut aggregate_event_list = AggregateEventList::new();
        match self {
            Self::Created(node_ids) => {
                // we add nodes to adjacency list map first
                node_changes.for_each_add(|change| {
                    adjacency.add(change);
                });

                // then we build connected set and save aggregates
                for node_id in node_ids.iter() {
                    let connected_set = NodeConnectedSet::new(node_id, &adjacency)?;
                    let agg = aggregates.save(connected_set, node_entities, order, None)?;
                    aggregate_event_list.extend(agg);
                }
            }
            Self::Deleted(node_ids) => {
                // we keep adjacency list map intact to build connected set
                // to match which aggregate can be deleted first
                for node_id in node_ids.iter() {
                    let connected_set = NodeConnectedSet::new(node_id, &adjacency)?;
                    let agg =
                        aggregates.save(connected_set, node_entities, order, Some(node_changes))?;
                    aggregate_event_list.extend(agg);
                }

                // then we delete nodes from adjacency list map
                node_changes.for_each_delete(|change| {
                    adjacency.remove(change);
                });
            }
        }

        Ok(aggregate_event_list)
    }
}
