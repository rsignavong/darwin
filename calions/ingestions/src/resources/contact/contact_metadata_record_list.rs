use super::ContactMetadataRecord;
use crate::resources::RecordValue;
use anyhow::Error as AnyError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContactMetadataRecordList(Vec<ContactMetadataRecord>);

impl ContactMetadataRecordList {
    pub fn add(&mut self, record: ContactMetadataRecord, appendable: bool) {
        if appendable {
            let mut to_delete = None;
            for r in self.0.iter() {
                if r.value() == record.value() {
                    if r.transaction_id() < record.transaction_id() {
                        to_delete = Some(r.clone());
                        break;
                    } else {
                        return;
                    }
                }
            }
            if let Some(ref to_delete) = to_delete {
                self.0.retain(|r| r != to_delete);
            }
            self.0.push(record);
            self.0
                .sort_unstable_by(|a, b| b.transaction_id().cmp(a.transaction_id()))
        } else {
            let mut latest = record;
            for r in self.0.iter() {
                if r.transaction_id() > latest.transaction_id() {
                    latest = r.clone();
                }
            }
            self.0 = vec![latest];
        }
    }

    pub fn iter_each<F>(&self, mut func: F)
    where
        F: FnMut(&ContactMetadataRecord),
    {
        for r in self.0.iter() {
            func(r);
        }
    }

    pub fn iter_mut_each<F>(&mut self, func: F) -> Result<(), AnyError>
    where
        F: Fn(&mut ContactMetadataRecord) -> Result<(), AnyError>,
    {
        for r in self.0.iter_mut() {
            func(r)?;
        }

        Ok(())
    }

    pub fn new(record: ContactMetadataRecord) -> Self {
        ContactMetadataRecordList(vec![record])
    }

    pub fn latest_value(&self) -> Option<&Arc<RecordValue>> {
        self.0.last().map(|r| r.value())
    }

    pub fn unique_values(&self) -> Vec<Arc<RecordValue>> {
        self.0.iter().map(|r| r.value().clone()).collect()
    }
}
