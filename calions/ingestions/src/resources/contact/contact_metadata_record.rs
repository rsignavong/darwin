use crate::resources::{RecordValue, TransactionId};
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, new)]
pub struct ContactMetadataRecord {
    transaction_id: Arc<TransactionId>,
    value: Arc<RecordValue>,
}

impl ContactMetadataRecord {
    pub fn set_value(&mut self, value: Arc<RecordValue>) {
        self.value = value;
    }

    pub fn transaction_id(&self) -> &Arc<TransactionId> {
        &self.transaction_id
    }

    pub fn value(&self) -> &Arc<RecordValue> {
        &self.value
    }
}
