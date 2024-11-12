use crate::resources::IngestionFieldsSet;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize, new)]
pub struct IngestionDataMapping {
    pub appendable_fields: Arc<IngestionFieldsSet>,
    pub gdpr_fields: Arc<IngestionFieldsSet>,
}
