use crate::resources::{Contact, ContactDateTime, ContactId, ContactType};
use crate::resources::{ContactData, ContactMetadata as ContactMetadataData};
use crate::resources::{MappingId, OrganizationId};
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct ContactBody {
    data: Arc<ContactData>,
    id: Arc<ContactId>,
    organization_id: Arc<OrganizationId>,
    inserted_at: Arc<ContactDateTime>,
    updated_at: Arc<ContactDateTime>,
}

#[derive(Debug, Serialize)]
pub struct ContactMetadata {
    activated_mapping_id: Arc<MappingId>,
    sources: Arc<ContactMetadataData>,
}

impl OutputStream<ContactBody, ContactMetadata> for Contact {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        match self.type_ {
            ContactType::Created => StreamEvent::Created,
            ContactType::Updated => StreamEvent::Updated,
            ContactType::Deleted => StreamEvent::Deleted,
        }
    }

    fn body(&self) -> HashMap<u64, ContactBody> {
        let data = ContactBody {
            data: self.data.clone(),
            id: self.id.clone(),
            organization_id: self.organization_id.clone(),
            inserted_at: self.inserted_at.clone(),
            updated_at: self.updated_at.clone(),
        };

        let mut body = HashMap::new();
        body.insert(1, data);
        body
    }

    fn metadata(&self) -> Option<ContactMetadata> {
        let metadata = ContactMetadata {
            activated_mapping_id: self.mapping_id.clone(),
            sources: self.metadata.clone(),
        };

        Some(metadata)
    }
}
