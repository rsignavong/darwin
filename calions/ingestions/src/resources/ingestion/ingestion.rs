use super::{IngestionError, IngestionFieldsSet};
use crate::applications::PostgreSql;
use crate::decoders::ActivatedMappingBody;
use crate::decoders::AnonymizationBody;
use crate::decoders::{IngestionDataBody, IngestionDataMetadata};
use crate::decoders::{ReconciliationRecordBody, ReconciliationRecordMetadata};
use crate::resources::{Contact, ContactCount, ContactModel, ContactType};
use crate::resources::{ContactData, ContactDataRecord};
use crate::resources::{ContactMetadata, ContactMetadataRecord};
use crate::resources::{GdprEncrypter, GdprKey, GdprKeyDataGroup, GdprKeyModel, GdprKeyType};
use crate::resources::{IngestionData, IngestionDataMapping, IngestionDataRecord};
use crate::resources::{MappingId, OrganizationId, ProcessorId};
use crate::Settings;
use log::{info, warn};
use smol::Task;
use std::collections::HashSet;
use std::mem::take;
use std::sync::Arc;

pub struct Ingestion {
    contacts_count: ContactCount,
    appendable_fields: IngestionFieldsSet,
    gdpr_fields: IngestionFieldsSet,
    mapping_id: Option<Arc<MappingId>>,
    organization_id: Arc<OrganizationId>,
    postgresql: PostgreSql,
    processor_id: Arc<ProcessorId>,
}

impl Ingestion {
    pub async fn anonymize_contact(
        &mut self,
        anonymization_body: &AnonymizationBody,
    ) -> Result<Option<GdprKey>, IngestionError> {
        if !self.organization_id.eq(&anonymization_body.organization_id) {
            warn!(
                "Wrong OrganizationId {:?}, expecting {:?}",
                anonymization_body.organization_id, self.organization_id
            );
            return Ok(None);
        }

        if !anonymization_body.is_validated.is_validated() {
            return Ok(None);
        }

        let contact_id = anonymization_body.contact_id.as_ref();

        let mut client = self.postgresql.client().await?;
        let transaction = client.transaction().await.map_err(|source| {
            IngestionError::IngestionAnonymizationPostgresTransactionNew { source }
        })?;
        let contact_fetch_one_stmt = ContactModel::stmt_fetch_one(&transaction).await?;
        let contact_update_stmt = ContactModel::stmt_update(&transaction).await?;
        let gdpr_keys_fetch_stmt = GdprKeyModel::stmt_fetch_by_contact_id(&transaction).await?;
        let gdpr_keys_delete_stmt = GdprKeyModel::stmt_delete_by_contact_id(&transaction).await?;

        let gdpr_key =
            GdprKeyModel::fetch_one(&transaction, &gdpr_keys_fetch_stmt, contact_id).await?;
        let contact_model =
            ContactModel::fetch_one(&transaction, &contact_fetch_one_stmt, contact_id).await?;
        if let Some(mut contact_model) = contact_model {
            contact_model.remove_gdpr_items(&self.gdpr_fields);
            contact_model
                .update(&transaction, &contact_update_stmt)
                .await?;
        }

        if let Some(ref gdpr_key) = gdpr_key {
            gdpr_key
                .delete(&transaction, &gdpr_keys_delete_stmt)
                .await?;
        }

        transaction.commit().await.map_err(|source| {
            IngestionError::IngestionAnonymizationPostgresTransactionUncommitted { source }
        })?;

        self.contacts_count.dec();

        info!(
            "Contact Id {:?} successfully anonymized",
            anonymization_body.contact_id.to_string()
        );

        let key = gdpr_key.map(|gdpr_key| {
            GdprKey::new(
                GdprKeyType::Deleted,
                Arc::new(gdpr_key),
                self.organization_id.clone(),
            )
        });

        Ok(key)
    }

    pub fn count(&self) -> Arc<ContactCount> {
        Arc::new(self.contacts_count.clone())
    }

    pub async fn create(
        &mut self,
        record_body: &ReconciliationRecordBody,
        record_metadata: &ReconciliationRecordMetadata,
    ) -> Result<Option<(Contact, GdprKey)>, IngestionError> {
        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| IngestionError::IngestionCreateMissingActivatedMappingId)?;

        if !mapping_id.eq(&record_metadata.activated_mapping_id) {
            warn!(
                "Wrong ActivatedMappingId {:?}, expecting {:?}",
                record_metadata.activated_mapping_id, mapping_id
            );
            return Ok(None);
        }

        let mut client = self.postgresql.client().await?;
        let transaction = client
            .transaction()
            .await
            .map_err(|source| IngestionError::IngestionCreatePostgresTransactionNew { source })?;
        let create_contact_stmt = ContactModel::statement_create(&transaction).await?;
        let create_key_stmt = GdprKeyModel::statement_create(&transaction).await?;

        let mut new_contact = ContactModel::new(
            record_body.id.clone(),
            ContactData::new(),
            ContactMetadata::new(),
            self.organization_id.clone(),
        );

        for (field, data) in record_body.record.iter() {
            let appendable = self.appendable_fields.contains(field);
            match data {
                ContactDataRecord::Append(values) => {
                    for value in values {
                        let metadata = ContactMetadataRecord::new(
                            record_metadata.transaction_id.clone(),
                            value.clone(),
                        );

                        new_contact.upsert_metadata(field.clone(), metadata, appendable);
                    }
                }
                ContactDataRecord::Detach(value) => {
                    let metadata = ContactMetadataRecord::new(
                        record_metadata.transaction_id.clone(),
                        value.clone(),
                    );
                    new_contact.upsert_metadata(field.clone(), metadata, appendable);
                }
            }
        }

        new_contact.update_data(&self.appendable_fields)?;

        let new_key =
            GdprKeyModel::new(new_contact.id.clone(), GdprKeyDataGroup::IngestedContact, 1)?;

        info!("Create contact & key {:?}", new_contact.id.to_string());
        let mut new_contact_result = new_contact
            .create(&transaction, &create_contact_stmt)
            .await?;
        let new_key_result = new_key.create(&transaction, &create_key_stmt).await?;

        transaction.commit().await.map_err(|source| {
            IngestionError::IngestionCreatePostgresTransactionUncommitted { source }
        })?;

        self.contacts_count.inc();

        GdprEncrypter::encrypt(
            &mut new_contact_result,
            new_key.key.inner(),
            &self.gdpr_fields,
        )?;

        let contact = Contact::new(
            ContactType::Created,
            new_contact_result.id.clone(),
            Arc::new(take(&mut new_contact_result.data)),
            Arc::new(take(&mut new_contact_result.metadata)),
            mapping_id.clone(),
            new_contact_result.organization_id.clone(),
            new_contact_result.inserted_at.clone(),
            new_contact_result.updated_at.clone(),
            Arc::new(self.contacts_count.clone()),
        );

        let key = GdprKey::new(
            GdprKeyType::Created,
            Arc::new(new_key_result),
            self.organization_id.clone(),
        );

        Ok(Some((contact, key)))
    }

    pub fn is_ready(&self) -> bool {
        let ready = self.mapping_id.is_some();
        info!(
            "Ingestions is {}",
            if ready { "ready" } else { "not ready..." }
        );

        ready
    }

    pub fn mapping_id(&self) -> Result<Arc<MappingId>, IngestionError> {
        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| IngestionError::IngestionMissingActivatedMappingId)?;
        Ok(mapping_id)
    }

    pub async fn new(
        processor_id: Arc<ProcessorId>,
        organization_id: Arc<OrganizationId>,
    ) -> Result<Self, IngestionError> {
        let postgresql = PostgreSql::new();
        let pg = postgresql.clone();
        let client = Task::spawn(async move { pg.client().await }).await?;
        let org_id = organization_id.clone();
        let contacts_count =
            Task::spawn(async move { ContactModel::count(&client, org_id).await }).await?;

        info!("Starting with {} ingested contacts", contacts_count);
        Ok(Ingestion {
            mapping_id: None,
            contacts_count,
            appendable_fields: HashSet::new(),
            gdpr_fields: HashSet::new(),
            organization_id,
            postgresql,
            processor_id,
        })
    }

    pub fn set_mapping(
        &mut self,
        amb: &ActivatedMappingBody,
    ) -> Result<Option<IngestionDataRecord>, IngestionError> {
        let awaited_mapping_id = Arc::new(Settings::get().activated_mapping_id.clone());
        if !awaited_mapping_id.eq(&amb.id) {
            return Ok(None);
        }

        if self.mapping_id.is_none() {
            self.mapping_id = Some(amb.id.clone());
        }

        for map in amb.mappings.iter() {
            if *map.is_gdpr {
                self.gdpr_fields.insert(map.field_alias.clone());
            }

            if *map.is_appendable {
                self.appendable_fields.insert(map.field_alias.clone());
            }
        }

        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| IngestionError::IngestionSetMappingMissingActivatedMappingId)?;

        info!("Activated Mapping: {}", mapping_id);

        let mapping = IngestionDataMapping::new(
            Arc::new(self.appendable_fields.clone()),
            Arc::new(self.gdpr_fields.clone()),
        );
        let ingestion_data_record =
            IngestionDataRecord::new(IngestionData::Mapping(mapping), self.processor_id.clone());

        Ok(Some(ingestion_data_record))
    }

    pub fn set_state(
        &mut self,
        idb: &IngestionDataBody,
        idm: &IngestionDataMetadata,
    ) -> Result<(), IngestionError> {
        if !self.processor_id.eq(&idm.processor_id) {
            return Ok(());
        }

        match idb {
            IngestionDataBody::Mapping(ref mapping) => {
                self.appendable_fields = mapping.appendable_fields.as_ref().clone();
                self.gdpr_fields = mapping.gdpr_fields.as_ref().clone();
            }
        }

        self.mapping_id = Some(Arc::new(Settings::get().activated_mapping_id.clone()));

        Ok(())
    }

    pub async fn update(
        &mut self,
        record_body: &ReconciliationRecordBody,
        record_metadata: &ReconciliationRecordMetadata,
    ) -> Result<Option<(Contact, Vec<(Contact, GdprKey)>)>, IngestionError> {
        let mapping_id = self
            .mapping_id
            .clone()
            .ok_or_else(|| IngestionError::IngestionUpdateMissingActivatedMappingId)?;

        if !mapping_id.eq(&record_metadata.activated_mapping_id) {
            warn!(
                "Wrong ActivatedMappingId {:?}, expecting {:?}",
                record_metadata.activated_mapping_id, mapping_id
            );
            return Ok(None);
        }

        let mut ids = vec![record_body.id.as_ref().clone()];
        ids.extend(record_metadata.merges.as_ref().clone());

        let mut client = self.postgresql.client().await?;
        let transaction = client
            .transaction()
            .await
            .map_err(|source| IngestionError::IngestionUpdatePostgresTransactionNew { source })?;
        let stmts = ContactModel::statements(&transaction).await?;
        let key_stmts = GdprKeyModel::statements(&transaction).await?;
        let mut contact_models = ContactModel::fetch(&transaction, stmts.fetch(), &ids).await?;
        let mut key_models = GdprKeyModel::fetch(&transaction, key_stmts.fetch(), &ids).await?;
        if contact_models.is_empty() {
            return Ok(None);
        }
        let contacts_to_merge: Vec<ContactModel> = contact_models.drain(1..).collect();
        let keys_to_delete: Vec<Arc<GdprKeyModel>> = key_models.drain(1..).map(Arc::new).collect();
        let mut contact_to_update = contact_models
            .pop()
            .ok_or_else(|| IngestionError::IngestionUpdateNoContact)?;
        let contact_key = key_models
            .pop()
            .ok_or_else(|| IngestionError::IngestionUpdateNoContact)?;

        for (field, data) in record_body.record.iter() {
            let appendable = self.appendable_fields.contains(field);
            match data {
                ContactDataRecord::Append(values) => {
                    for value in values {
                        let metadata = ContactMetadataRecord::new(
                            record_metadata.transaction_id.clone(),
                            value.clone(),
                        );
                        contact_to_update.upsert_metadata(field.clone(), metadata, appendable);
                    }
                }
                ContactDataRecord::Detach(value) => {
                    let metadata = ContactMetadataRecord::new(
                        record_metadata.transaction_id.clone(),
                        value.clone(),
                    );
                    contact_to_update.upsert_metadata(field.clone(), metadata, appendable);
                }
            }
        }

        for contact_to_merge in contacts_to_merge.iter() {
            for (field, metadata_list) in contact_to_merge.metadata.iter() {
                let appendable = self.appendable_fields.contains(field);
                metadata_list.iter_each(|metadata| {
                    contact_to_update.upsert_metadata(field.clone(), metadata.clone(), appendable);
                });
            }
        }

        contact_to_update.update_data(&self.appendable_fields)?;

        info!("Update contact {:?}", contact_to_update.id);
        contact_to_update
            .update(&transaction, stmts.update())
            .await?;
        if !record_metadata.merges.is_empty() {
            info!("Delete contacts & keys in {:?}", record_metadata.merges);
            // Delete contacts deletes keys in cascade
            ContactModel::delete_many(
                &transaction,
                stmts.delete(),
                record_metadata.merges.as_ref(),
            )
            .await?;
        }
        transaction.commit().await.map_err(|source| {
            IngestionError::IngestionUpdatePostgresTransactionUncommitted { source }
        })?;

        let mut contacts: Vec<(Contact, GdprKey)> = Vec::new();
        for (index, mut contact) in contacts_to_merge.into_iter().enumerate() {
            self.contacts_count.dec();

            GdprEncrypter::encrypt(
                &mut contact,
                keys_to_delete[index].key.inner(),
                &self.gdpr_fields,
            )?;

            let contact = Contact::new(
                ContactType::Deleted,
                contact.id.clone(),
                Arc::new(take(&mut contact.data)),
                Arc::new(take(&mut contact.metadata)),
                mapping_id.clone(),
                contact.organization_id.clone(),
                contact.inserted_at.clone(),
                contact.updated_at.clone(),
                Arc::new(self.contacts_count.clone()),
            );
            let key = GdprKey::new(
                GdprKeyType::Deleted,
                keys_to_delete[index].clone(),
                self.organization_id.clone(),
            );
            contacts.push((contact, key));
        }

        GdprEncrypter::encrypt(
            &mut contact_to_update,
            contact_key.key.inner(),
            &self.gdpr_fields,
        )?;

        let contact = Contact::new(
            ContactType::Updated,
            contact_to_update.id.clone(),
            Arc::new(take(&mut contact_to_update.data)),
            Arc::new(take(&mut contact_to_update.metadata)),
            mapping_id.clone(),
            contact_to_update.organization_id.clone(),
            contact_to_update.inserted_at.clone(),
            contact_to_update.updated_at.clone(),
            Arc::new(self.contacts_count.clone()),
        );

        Ok(Some((contact, contacts)))
    }
}
