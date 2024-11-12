use crate::resources::{AnonymizationValidated, ContactId, OrganizationId};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize)]
pub struct AnonymizationMetadata;

#[derive(Clone, Debug, Deserialize)]
pub struct AnonymizationBody {
    pub contact_id: Arc<ContactId>,
    pub is_validated: Arc<AnonymizationValidated>,
    pub organization_id: Arc<OrganizationId>,
}
