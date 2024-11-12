use super::{ContactMetadataItem, ContactMetadataRecord};
use crate::resources::{RecordTime, RecordValue};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ContactMetadataItemType {
    Append(Vec<Arc<ContactMetadataItem>>),
    Detach(Arc<ContactMetadataItem>),
}

impl ContactMetadataItemType {
    pub fn append_or_self(&self) -> Self {
        if let Self::Detach(item) = self {
            return ContactMetadataItemType::Append(vec![item.clone()]);
        }

        self.clone()
    }

    pub fn detach_or_self(&self) -> Option<Self> {
        if let Self::Append(items) = self {
            let last = items.last()?;
            return Some(ContactMetadataItemType::Detach(last.clone()));
        }

        Some(self.clone())
    }

    pub fn set(
        &mut self,
        item: Self,
        timestamp: Arc<RecordTime>,
        previous_timestamp: Option<Arc<RecordTime>>,
    ) {
        match (self, item) {
            (Self::Append(self_items), Self::Detach(ref item)) => {
                self_items.push(item.clone());
            }
            (Self::Append(self_items), Self::Append(ref items)) => {
                self_items.extend(items.clone());
            }
            (Self::Detach(self_item), Self::Detach(ref item)) => {
                if let Some(prev_timestamp) = previous_timestamp {
                    if timestamp > prev_timestamp {
                        *self_item = item.clone();
                    }
                }
            }
            (Self::Detach(self_item), Self::Append(ref items)) => {
                if let Some(last_item) = items.clone().pop() {
                    *self_item = last_item;
                }
            }
        };
    }

    pub fn metadata_record(&self, value: &Arc<RecordValue>) -> Option<&ContactMetadataRecord> {
        match self {
            Self::Append(items) => {
                for item in items.iter() {
                    if item.is_same(value) {
                        return item.last_metadata_record_by_timestamp();
                    }
                }

                None
            }
            Self::Detach(item) => {
                if item.is_same(value) {
                    item.last_metadata_record_by_timestamp()
                } else {
                    None
                }
            }
        }
    }
}
