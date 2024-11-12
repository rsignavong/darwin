use super::{ContactCount, ContactDateTime, ContactId, ContactModelStatements};
use super::{ContactData, ContactDataRecord, ContactError};
use super::{ContactMetadata, ContactMetadataRecord, ContactMetadataRecordList};
use crate::resources::{IngestionFieldsSet, MappingField, OrganizationId};
use crate::Settings;
use chrono::NaiveDateTime;
use deadpool_postgres::{Client, Transaction};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use tokio_postgres::types::{Json, Type};
use tokio_postgres::{Row, Statement};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactModel {
    pub id: Arc<ContactId>,
    pub data: ContactData,
    pub metadata: ContactMetadata,
    pub organization_id: Arc<OrganizationId>,
    pub inserted_at: Arc<ContactDateTime>,
    pub updated_at: Arc<ContactDateTime>,
}

impl TryFrom<Row> for ContactModel {
    type Error = ContactError;

    fn try_from(r: Row) -> Result<Self, Self::Error> {
        let id = r
            .try_get::<usize, &str>(0)
            .map_err(|source| ContactError::ContactModelTryFromRowId { source })?
            .try_into()?;
        let data = r
            .try_get::<usize, Json<ContactData>>(1)
            .map_err(|source| ContactError::ContactModelTryFromRowData { source })?
            .0;
        let metadata = r
            .try_get::<usize, Json<ContactMetadata>>(2)
            .map_err(|source| ContactError::ContactModelTryFromRowMetadata { source })?
            .0;
        let organization_id = r
            .try_get::<usize, Uuid>(3)
            .map_err(|source| ContactError::ContactModelTryFromRowOrganizationId { source })?
            .into();
        let inserted_at = r
            .try_get::<usize, NaiveDateTime>(4)
            .map_err(|source| ContactError::ContactModelTryFromRowInsertedAt { source })?
            .into();
        let updated_at = r
            .try_get::<usize, NaiveDateTime>(5)
            .map_err(|source| ContactError::ContactModelTryFromRowUpdatedAt { source })?
            .into();

        Ok(ContactModel {
            id: Arc::new(id),
            data,
            metadata,
            organization_id: Arc::new(organization_id),
            inserted_at: Arc::new(inserted_at),
            updated_at: Arc::new(updated_at),
        })
    }
}

impl ContactModel {
    pub async fn count(
        client: &Client,
        organization_id: Arc<OrganizationId>,
    ) -> Result<ContactCount, ContactError> {
        let stmt = Self::stmt_count(&client).await?;
        client
            .query_one(&stmt, &[&**organization_id])
            .await
            .map_err(|source| ContactError::ContactModelQueryCount { source })
            .and_then(ContactCount::try_from)
    }

    pub async fn create(
        &self,
        transaction: &Transaction<'_>,
        stmt: &Statement,
    ) -> Result<Self, ContactError> {
        transaction
            .query_one(
                stmt,
                &[
                    &self.id.to_string(),
                    &Json(&self.data),
                    &Json(&self.metadata),
                    &**self.organization_id,
                    &**self.inserted_at,
                    &**self.updated_at,
                ],
            )
            .await
            .map_err(|source| ContactError::ContactModelQueryCreate { source })
            .and_then(ContactModel::try_from)
    }

    pub async fn delete_many(
        transaction: &Transaction<'_>,
        stmt: &Statement,
        contact_ids: &[ContactId],
    ) -> Result<u64, ContactError> {
        let ids: Vec<String> = contact_ids.iter().map(|i| i.to_string()).collect();
        transaction
            .execute(stmt, &[&ids])
            .await
            .map_err(|source| ContactError::ContactModelQueryDelete { source })
    }

    pub async fn fetch(
        transaction: &Transaction<'_>,
        stmt: &Statement,
        contact_ids: &[ContactId],
    ) -> Result<Vec<Self>, ContactError> {
        let ids: Vec<String> = contact_ids.iter().map(|i| i.to_string()).collect();
        transaction
            .query(stmt, &[&ids])
            .await
            .map_err(|source| ContactError::ContactModelQueryFetch { source })
            .and_then(|rows| {
                rows.into_iter()
                    .map(ContactModel::try_from)
                    .collect::<Result<Vec<ContactModel>, ContactError>>()
            })
    }

    pub async fn fetch_one(
        transaction: &Transaction<'_>,
        stmt: &Statement,
        contact_id: &ContactId,
    ) -> Result<Option<Self>, ContactError> {
        transaction
            .query_opt(stmt, &[&contact_id.to_string()])
            .await
            .map_err(|source| ContactError::ContactModelQueryFetchOne { source })
            .and_then(|row| row.map(|r| r.try_into()).transpose())
    }

    pub fn new(
        id: Arc<ContactId>,
        data: ContactData,
        metadata: ContactMetadata,
        organization_id: Arc<OrganizationId>,
    ) -> Self {
        let now = Arc::new(ContactDateTime::new());

        ContactModel {
            id,
            data,
            metadata,
            organization_id,
            inserted_at: now.clone(),
            updated_at: now,
        }
    }

    pub fn remove_gdpr_items(&mut self, gdpr_fields: &IngestionFieldsSet) {
        for field in gdpr_fields.iter() {
            self.data.remove(field);
            self.metadata.remove(field);
        }
    }

    pub async fn statement_create(
        transaction: &Transaction<'_>,
    ) -> Result<Statement, ContactError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    INSERT INTO {} ( id, data, metadata, organization_id, inserted_at, updated_at)
                    VALUES ( $1, $2, $3, $4, $5, $6 )
                    RETURNING *
                    "#,
                    Settings::get().postgresql.tables.contacts.table.to_string()
                )
                .as_str(),
                &[
                    Type::VARCHAR,
                    Type::JSON,
                    Type::JSON,
                    Type::UUID,
                    Type::TIMESTAMP,
                    Type::TIMESTAMP,
                ],
            )
            .await
            .map_err(|source| ContactError::ContactModelStatementCreate { source })
    }

    pub async fn statements(
        transaction: &Transaction<'_>,
    ) -> Result<ContactModelStatements, ContactError> {
        let delete = Self::stmt_delete(transaction).await?;
        let fetch = Self::stmt_fetch(transaction).await?;
        let update = Self::stmt_update(transaction).await?;

        Ok(ContactModelStatements::new(delete, fetch, update))
    }

    pub async fn stmt_fetch_one(transaction: &Transaction<'_>) -> Result<Statement, ContactError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    SELECT *
                    FROM {}
                    WHERE id = $1
                    "#,
                    Settings::get().postgresql.tables.contacts.table.to_string()
                )
                .as_str(),
                &[Type::VARCHAR],
            )
            .await
            .map_err(|source| ContactError::ContactModelStatementFetchOne { source })
    }

    pub async fn stmt_update(transaction: &Transaction<'_>) -> Result<Statement, ContactError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    UPDATE {}
                    SET data = $2, metadata = $3, updated_at = $4
                    WHERE id = $1
                    "#,
                    Settings::get().postgresql.tables.contacts.table.to_string()
                )
                .as_str(),
                &[Type::VARCHAR, Type::JSON, Type::JSON, Type::TIMESTAMP],
            )
            .await
            .map_err(|source| ContactError::ContactModelStatementUpdate { source })
    }

    async fn stmt_count(client: &Client) -> Result<Statement, ContactError> {
        client
            .prepare_typed(
                format!(
                    r#"
                    SELECT COUNT(id) as count
                    FROM {}
                    WHERE organization_id = $1
                    "#,
                    Settings::get().postgresql.tables.contacts.table.to_string()
                )
                .as_str(),
                &[Type::UUID],
            )
            .await
            .map_err(|source| ContactError::ContactModelStatementCount { source })
    }

    async fn stmt_delete(transaction: &Transaction<'_>) -> Result<Statement, ContactError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    DELETE FROM {}
                    WHERE id = ANY($1)
                    "#,
                    Settings::get().postgresql.tables.contacts.table.to_string()
                )
                .as_str(),
                &[Type::VARCHAR_ARRAY],
            )
            .await
            .map_err(|source| ContactError::ContactModelStatementDelete { source })
    }

    async fn stmt_fetch(transaction: &Transaction<'_>) -> Result<Statement, ContactError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    SELECT c.*
                    FROM unnest($1) WITH ORDINALITY AS ids_list(id, ord),
                    {} AS c
                    WHERE c.id = ids_list.id
                    ORDER BY ids_list.ord
                    "#,
                    Settings::get().postgresql.tables.contacts.table.to_string()
                )
                .as_str(),
                &[Type::VARCHAR_ARRAY],
            )
            .await
            .map_err(|source| ContactError::ContactModelStatementFetch { source })
    }

    pub async fn update(
        &self,
        transaction: &Transaction<'_>,
        stmt: &Statement,
    ) -> Result<(), ContactError> {
        transaction
            .execute(
                stmt,
                &[
                    &self.id.to_string(),
                    &Json(&self.data),
                    &Json(&self.metadata),
                    &**self.updated_at,
                ],
            )
            .await
            .map_err(|source| ContactError::ContactModelQueryUpdate { source })?;
        Ok(())
    }

    pub fn update_data(
        &mut self,
        appendable_fields: &IngestionFieldsSet,
    ) -> Result<(), ContactError> {
        for (field, metadata_list) in self.metadata.iter() {
            let data = if appendable_fields.contains(field) {
                ContactDataRecord::Append(metadata_list.unique_values())
            } else {
                let value = metadata_list.latest_value().ok_or_else(|| {
                    ContactError::ContactModelUpdateDataFromLatestMetadata(field.clone())
                })?;
                ContactDataRecord::Detach(value.clone())
            };
            self.data.insert(field.clone(), data);
        }

        Ok(())
    }

    pub fn upsert_metadata(
        &mut self,
        field: Arc<MappingField>,
        metadata: ContactMetadataRecord,
        appendable: bool,
    ) {
        if metadata.value().is_empty() {
            return;
        }

        self.metadata
            .entry(field)
            .and_modify(|l| {
                l.add(metadata.clone(), appendable);
            })
            .or_insert(ContactMetadataRecordList::new(metadata));
    }
}
