use crate::resources::EntityRecordRelationship;
use std::sync::Arc;

pub struct EntitiyRecordRelationshipList(Vec<Arc<EntityRecordRelationship>>);

impl EntitiyRecordRelationshipList {
    pub fn add(&mut self, relationship: Arc<EntityRecordRelationship>) {
        self.0.push(relationship);
    }

    pub fn for_each<F>(self, mut func: F)
    where
        F: FnMut(Arc<EntityRecordRelationship>),
    {
        for rel in self.0.into_iter() {
            func(rel);
        }
    }

    pub fn new(relationship: Arc<EntityRecordRelationship>) -> Self {
        EntitiyRecordRelationshipList(vec![relationship])
    }
}
