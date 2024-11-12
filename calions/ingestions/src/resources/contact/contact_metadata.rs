use super::ContactMetadataRecordList;
use crate::resources::MappingField;
use std::collections::HashMap;
use std::sync::Arc;

pub type ContactMetadata = HashMap<Arc<MappingField>, ContactMetadataRecordList>;
