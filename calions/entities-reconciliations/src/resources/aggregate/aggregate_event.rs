use super::{AggregateEntry, AggregateId};
use crate::resources::Entity;
use std::sync::Arc;

#[derive(Debug)]
pub enum AggregateEvent {
    Created(AggregateEntry),
    Deleted(AggregateEntry),
}

impl AggregateEvent {
    pub fn id(&self) -> &Arc<AggregateId> {
        match self {
            Self::Created(entry) | Self::Deleted(entry) => entry.id(),
        }
    }

    pub fn reconciliations(&self) -> Vec<Arc<Entity>> {
        match self {
            Self::Created(entry) | Self::Deleted(entry) => {
                entry.reconciliations().unwrap().iter().cloned().collect()
            }
        }
    }
}
