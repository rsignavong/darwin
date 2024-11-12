use super::GraphNodeEventUpdate;
use crate::resources::NodeId;
use std::sync::Arc;

pub enum GraphNodeEvent {
    Created(Arc<NodeId>),
    Deleted(Arc<NodeId>),
    Ignored(Arc<NodeId>),
    Updated(GraphNodeEventUpdate),
}

impl GraphNodeEvent {
    pub fn created(&self) -> Option<&Arc<NodeId>> {
        match self {
            Self::Created(node_id) => Some(node_id),
            Self::Updated(update) => Some(update.new_id()),
            _ => None,
        }
    }

    pub fn deleted(&self) -> Option<&Arc<NodeId>> {
        match self {
            Self::Deleted(node_id) => Some(node_id),
            Self::Updated(update) => Some(update.old_id()),
            _ => None,
        }
    }
}
