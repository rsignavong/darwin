use super::{GraphNodeEvent, GraphNodeEventUpdate};
use crate::resources::{MappingEntityMethod, NodeId};
use ahash::AHashSet;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum GraphNode {
    Append(AHashSet<Arc<NodeId>>),
    Fixed(Arc<NodeId>),
    Upsert(Arc<NodeId>),
}

impl GraphNode {
    pub fn contains(&self, node_id: &Arc<NodeId>) -> bool {
        match self {
            Self::Append(node_ids) => node_ids.contains(node_id),
            Self::Fixed(node_id_) | Self::Upsert(node_id_) => node_id_ == node_id,
        }
    }

    pub fn delete_and_is_empty(&mut self, node_id: &Arc<NodeId>) -> bool {
        match self {
            Self::Append(node_ids) => {
                node_ids.remove(node_id);
                node_ids.is_empty()
            }
            _ => true,
        }
    }

    pub fn new(entity_method: &MappingEntityMethod, node_id: &Arc<NodeId>) -> Self {
        match entity_method {
            MappingEntityMethod::Append => {
                let mut set = AHashSet::new();
                set.insert(node_id.clone());
                Self::Append(set)
            }
            MappingEntityMethod::Fixed => Self::Fixed(node_id.clone()),
            MappingEntityMethod::Upsert => Self::Upsert(node_id.clone()),
        }
    }

    pub fn set(&mut self, node_id: Arc<NodeId>) -> GraphNodeEvent {
        match self {
            Self::Append(node_ids) => {
                if node_ids.insert(node_id.clone()) {
                    GraphNodeEvent::Created(node_id)
                } else {
                    GraphNodeEvent::Ignored(node_id)
                }
            }
            Self::Fixed(_) => GraphNodeEvent::Ignored(node_id),
            Self::Upsert(node_id_) => {
                if *node_id_ == node_id {
                    return GraphNodeEvent::Ignored(node_id);
                }

                let updated = GraphNodeEvent::Updated(GraphNodeEventUpdate::new(
                    node_id_.clone(),
                    node_id.clone(),
                ));
                *node_id_ = node_id;
                updated
            }
        }
    }

    pub fn unwrap(&self) -> Vec<Arc<NodeId>> {
        match self {
            Self::Append(ref node_ids) => node_ids.into_iter().map(|n| n.clone()).collect(),
            Self::Fixed(node_id) | Self::Upsert(node_id) => vec![node_id.clone()],
        }
    }
}
