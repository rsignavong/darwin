use super::GdprError;
use crate::resources::{ContactDataRecord, ContactModel};
use crate::resources::{IngestionFieldsSet, RecordValue};
use anyhow::Error as AnyError;
use gdpr::{GdprKey, GdprValue};
use std::sync::Arc;

pub struct GdprEncrypter;

impl GdprEncrypter {
    pub fn encrypt(
        contact: &mut ContactModel,
        key: &GdprKey,
        gdpr_fields: &IngestionFieldsSet,
    ) -> Result<(), GdprError> {
        for (field, data) in contact.data.iter_mut() {
            if !gdpr_fields.contains(field) {
                continue;
            }

            match data {
                ContactDataRecord::Append(values) => {
                    for value in values.iter_mut() {
                        *value = Arc::new(RecordValue::new(
                            GdprValue::encrypt_with_key(&value.to_string(), key)?.to_string(),
                        ));
                    }
                }
                ContactDataRecord::Detach(value) => {
                    *value = Arc::new(RecordValue::new(
                        GdprValue::encrypt_with_key(&value.to_string(), key)?.to_string(),
                    ));
                }
            }
        }

        for (field, metadata_list) in contact.metadata.iter_mut() {
            if !gdpr_fields.contains(field) {
                continue;
            }

            metadata_list
                .iter_mut_each(|metadata| -> Result<(), AnyError> {
                    let encrypted_value =
                        GdprValue::encrypt_with_key(&metadata.value().to_string(), key)?;
                    metadata.set_value(Arc::new(RecordValue::new(encrypted_value.to_string())));

                    Ok(())
                })
                .map_err(GdprError::GdprEncryptMetadata)?;
        }

        Ok(())
    }
}
