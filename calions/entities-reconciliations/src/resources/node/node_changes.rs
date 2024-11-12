use super::{NodeChange, NodeEventList, NodeId};
use crate::resources::GraphNodeChangeList;
use std::sync::Arc;

#[derive(Debug)]
pub struct NodeChanges {
    adds: Vec<NodeChange>,
    deletes: Vec<NodeChange>,
}

impl From<GraphNodeChangeList> for NodeChanges {
    fn from(changes: GraphNodeChangeList) -> Self {
        let mut adds: Vec<NodeChange> = Vec::new();
        let mut deletes: Vec<NodeChange> = Vec::new();
        changes.for_each(|change| {
            if let Some(node) = change.opposite() {
                for node_id in node.unwrap().into_iter() {
                    if let Some(created_node_id) = change.created_event() {
                        let node_change = NodeChange::new(created_node_id.clone(), node_id.clone());
                        adds.push(node_change);
                    }
                    if let Some(deleted_node_id) = change.deleted_event() {
                        let node_change = NodeChange::new(deleted_node_id.clone(), node_id);
                        deletes.push(node_change);
                    }
                }
            }
        });

        NodeChanges { adds, deletes }
    }
}

impl NodeChanges {
    pub fn for_each_add<F>(&self, mut func: F)
    where
        F: FnMut(&NodeChange),
    {
        for change in self.adds.iter() {
            func(change);
        }
    }

    pub fn for_each_delete<F>(&self, mut func: F)
    where
        F: FnMut(&NodeChange),
    {
        for change in self.deletes.iter() {
            func(change);
        }
    }

    pub fn unique_created_list(&self) -> NodeEventList {
        let mut created_list: Vec<Arc<NodeId>> = self
            .adds
            .iter()
            .map(|c| c.event_node_id().clone())
            .collect();
        created_list.sort_unstable();
        created_list.dedup();

        NodeEventList::Created(created_list)
    }

    pub fn unique_deleted_list(&self) -> NodeEventList {
        let mut deleted_list: Vec<Arc<NodeId>> = self
            .deletes
            .iter()
            .map(|c| c.event_node_id().clone())
            .collect();
        deleted_list.sort_unstable();
        deleted_list.dedup();

        NodeEventList::Deleted(deleted_list)
    }
}
