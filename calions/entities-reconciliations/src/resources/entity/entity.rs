use crate::resources::{EntityRecordId, EntityType};
use derive_new::new;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize, new)]
pub struct Entity {
    #[serde(rename = "type")]
    type_: Arc<EntityType>,
    id: EntityRecordId,
}

impl Entity {
    pub fn is_same_type(&self, type_: &EntityType) -> bool {
        self.type_.as_ref() == type_
    }

    pub fn get_type(&self) -> &EntityType {
        self.type_.as_ref()
    }

    pub fn type_(&self) -> &Arc<EntityType> {
        &self.type_
    }
}
