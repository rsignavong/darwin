use super::ContactId;
use crate::resources::{OrganizationId, RecordValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContactDataRecord {
    Append(Vec<Arc<RecordValue>>),
    Detach(Arc<RecordValue>),
}

impl From<Arc<ContactId>> for ContactDataRecord {
    fn from(contact_id: Arc<ContactId>) -> Self {
        ContactDataRecord::Detach(Arc::new(RecordValue::new(contact_id.to_string())))
    }
}

impl From<Arc<OrganizationId>> for ContactDataRecord {
    fn from(organization_id: Arc<OrganizationId>) -> Self {
        ContactDataRecord::Detach(Arc::new(RecordValue::new(organization_id.to_string())))
    }
}
