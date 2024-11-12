use crate::resources::EntityType;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct ReconciliationOrder(Vec<Arc<EntityType>>);

impl ReconciliationOrder {
    pub fn for_each_type<F>(&self, mut func: F)
    where
        F: FnMut(&Arc<EntityType>),
    {
        for type_ in self.0.iter() {
            func(type_);
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}
