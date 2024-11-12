use super::MappingField;
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct MappingFieldList(Vec<Arc<MappingField>>);

impl MappingFieldList {
    pub fn add(&mut self, field: Arc<MappingField>) {
        self.0.push(field);
    }

    pub fn contains(&mut self, field: &Arc<MappingField>) -> bool {
        self.0.contains(field)
    }

    pub fn iter_each<F>(&self, mut func: F)
    where
        F: FnMut(&Arc<MappingField>),
    {
        for field in self.0.iter() {
            func(field);
        }
    }
}
