use super::ContactDataRecord;
use crate::resources::MappingField;
use std::collections::HashMap;
use std::sync::Arc;

pub type ContactData = HashMap<Arc<MappingField>, ContactDataRecord>;
