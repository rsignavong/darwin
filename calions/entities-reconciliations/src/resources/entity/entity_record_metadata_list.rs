use super::EntityRecordMetadata;
use derive_new::new;

#[derive(new)]
pub struct EntityRecordMetadataList(#[new(default)] Vec<EntityRecordMetadata>);

impl EntityRecordMetadataList {
    pub fn add(&mut self, metadata: EntityRecordMetadata) {
        self.0.push(metadata);
    }

    pub fn for_each<F>(self, mut func: F)
    where
        F: FnMut(EntityRecordMetadata),
    {
        for metadata in self.0.into_iter() {
            func(metadata);
        }
    }
}
