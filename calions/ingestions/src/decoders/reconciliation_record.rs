use crate::resources::{ContactData, ContactId, MappingId, TransactionId};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationRecordBody {
    pub id: Arc<ContactId>,
    pub record: Arc<ContactData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ReconciliationRecordMetadata {
    pub activated_mapping_id: Arc<MappingId>,
    pub merges: Arc<Vec<ContactId>>,
    pub transaction_id: Arc<TransactionId>,
}
