use super::MappingField;
use crate::resources::EntityType;
use derive_new::new;
use std::sync::Arc;

#[derive(Clone, Debug, Eq, Hash, PartialEq, new)]
pub struct MappingFieldEntityType {
    field: Arc<MappingField>,
    type_: Arc<EntityType>,
}
