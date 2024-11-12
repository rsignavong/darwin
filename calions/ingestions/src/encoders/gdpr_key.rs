use crate::resources::{ContactId, OrganizationId};
use crate::resources::{GdprKey, GdprKeyAlgo, GdprKeyDateTime, GdprKeyId, GdprKeyKey, GdprKeyType};
use crate::resources::{GdprKeyDataGroup, GdprKeyVersion};
use anyhow::Error as AnyError;
use data_stream::stream::{OutputStream, StreamEvent};
use serde::Serialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize)]
pub struct GdprKeyBody {
    pub organization_id: Arc<OrganizationId>,
    pub contact_id: Arc<ContactId>,
    pub id: Arc<GdprKeyId>,
    pub key: Arc<GdprKeyKey>,
    pub algo: Arc<GdprKeyAlgo>,
    pub inserted_at: Arc<GdprKeyDateTime>,
    pub updated_at: Arc<GdprKeyDateTime>,
}

#[derive(Debug, Serialize)]
pub struct GdprKeyMetadata {
    pub data_group: Arc<GdprKeyDataGroup>,
    pub version: Arc<GdprKeyVersion>,
}

impl OutputStream<GdprKeyBody, GdprKeyMetadata> for GdprKey {
    fn key(&self) -> Result<String, AnyError> {
        Ok(self.key.id.to_string())
    }

    fn event(&self) -> StreamEvent {
        match self.type_ {
            GdprKeyType::Created => StreamEvent::Created,
            GdprKeyType::Deleted => StreamEvent::Deleted,
        }
    }

    fn body(&self) -> HashMap<u64, GdprKeyBody> {
        let data = GdprKeyBody {
            organization_id: self.organization_id.clone(),
            contact_id: self.key.contact_id.clone(),
            id: self.key.id.clone(),
            key: self.key.key.clone(),
            algo: self.key.algo.clone(),
            inserted_at: self.key.inserted_at.clone(),
            updated_at: self.key.updated_at.clone(),
        };

        let mut body = HashMap::new();
        body.insert(1, data);
        body
    }

    fn metadata(&self) -> Option<GdprKeyMetadata> {
        let metadata = GdprKeyMetadata {
            data_group: self.key.data_group.clone(),
            version: self.key.version.clone(),
        };

        Some(metadata)
    }
}
