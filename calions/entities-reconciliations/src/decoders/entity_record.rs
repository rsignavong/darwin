use crate::resources::{EntityRecord, MappingId};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct EntityRecordMetadata {
    pub activated_mapping_id: Option<Arc<MappingId>>,
}

pub type EntityRecordBody = Arc<EntityRecord>;
