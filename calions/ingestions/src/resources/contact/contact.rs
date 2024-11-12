use super::{ContactCount, ContactData, ContactDateTime, ContactId, ContactMetadata, ContactType};
use crate::resources::{MappingId, OrganizationId};
use derive_new::new;
use std::sync::Arc;

#[derive(new)]
pub struct Contact {
    pub type_: ContactType,
    pub id: Arc<ContactId>,
    pub data: Arc<ContactData>,
    pub metadata: Arc<ContactMetadata>,
    pub mapping_id: Arc<MappingId>,
    pub organization_id: Arc<OrganizationId>,
    pub inserted_at: Arc<ContactDateTime>,
    pub updated_at: Arc<ContactDateTime>,
    pub count: Arc<ContactCount>,
}
