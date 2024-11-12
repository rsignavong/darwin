use crate::resources::{MappingField, RecordValue};
use std::collections::HashMap;
use std::sync::Arc;

pub type RecordData = HashMap<Arc<MappingField>, Arc<RecordValue>>;

