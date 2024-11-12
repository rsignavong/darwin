use crate::resources::NodeId;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct GraphNodeEventUpdate {
    old: Arc<NodeId>,
    new: Arc<NodeId>,
}

impl GraphNodeEventUpdate {
    pub fn new_id(&self) -> &Arc<NodeId> {
        &self.new
    }

    pub fn old_id(&self) -> &Arc<NodeId> {
        &self.old
    }
}
