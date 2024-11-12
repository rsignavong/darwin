use crate::resources::Entity;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RecordReconciliations(Vec<Arc<Entity>>);

impl RecordReconciliations {
    pub fn add_entity(&mut self, entity: &Arc<Entity>) {
        self.0.push(entity.clone())
    }

    pub fn with_capacity(capacity: usize) -> Self {
        RecordReconciliations(Vec::with_capacity(capacity))
    }

    pub fn unwrap(&self) -> &[Arc<Entity>] {
        &self.0
    }
}
