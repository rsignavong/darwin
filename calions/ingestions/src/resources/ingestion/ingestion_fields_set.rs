use crate::resources::MappingField;
use std::collections::HashSet;
use std::sync::Arc;

pub type IngestionFieldsSet = HashSet<Arc<MappingField>>;

