use super::{GdprError, GdprKeyKey, GdprKeyModelStatements};
use super::{GdprKeyAlgo, GdprKeyDataGroup, GdprKeyDateTime, GdprKeyId, GdprKeyVersion};
use crate::resources::ContactId;
use crate::Settings;
use chrono::NaiveDateTime;
use deadpool_postgres::Transaction;
use derivative::Derivative;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use tokio_postgres::types::Type;
use tokio_postgres::{Row, Statement};

#[derive(Clone, Derivative)]
#[derivative(Debug)]
pub struct GdprKeyModel {
    pub id: Arc<GdprKeyId>,
    pub contact_id: Arc<ContactId>,
    pub data_group: Arc<GdprKeyDataGroup>,
    #[derivative(Debug = "ignore")]
    pub key: Arc<GdprKeyKey>,
    pub version: Arc<GdprKeyVersion>,
    pub algo: Arc<GdprKeyAlgo>,
    pub inserted_at: Arc<GdprKeyDateTime>,
    pub updated_at: Arc<GdprKeyDateTime>,
}

impl TryFrom<Row> for GdprKeyModel {
    type Error = GdprError;

    fn try_from(r: Row) -> Result<Self, Self::Error> {
        let id = r
            .try_get::<usize, &str>(0)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowId { source })?
            .try_into()?;
        let contact_id = r
            .try_get::<usize, &str>(1)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowContactId { source })?
            .try_into()
            .map_err(|source| GdprError::GdprKeyModelContactIdTryFrom { source })?;
        let data_group = r
            .try_get::<usize, &str>(2)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowDataGroup { source })?
            .try_into()?;
        let key = r
            .try_get::<usize, &[u8]>(3)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowKey { source })?
            .try_into()?;
        let version = ((r
            .try_get::<usize, i32>(4)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowVersion { source })?)
            as u32)
            .into();
        let algo = r
            .try_get::<usize, &str>(5)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowAlgo { source })?
            .try_into()?;
        let inserted_at = r
            .try_get::<usize, NaiveDateTime>(6)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowInsertedAt { source })?
            .into();
        let updated_at = r
            .try_get::<usize, NaiveDateTime>(7)
            .map_err(|source| GdprError::GdprKeyModelTryFromRowUpdatedAt { source })?
            .into();

        Ok(GdprKeyModel {
            id: Arc::new(id),
            contact_id: Arc::new(contact_id),
            data_group: Arc::new(data_group),
            key: Arc::new(key),
            version: Arc::new(version),
            algo: Arc::new(algo),
            inserted_at: Arc::new(inserted_at),
            updated_at: Arc::new(updated_at),
        })
    }
}

impl GdprKeyModel {
    pub async fn create(
        &self,
        transation: &Transaction<'_>,
        stmt: &Statement,
    ) -> Result<Self, GdprError> {
        transation
            .query_one(
                stmt,
                &[
                    &self.id.to_string(),
                    &self.contact_id.to_string(),
                    &self.data_group.to_string(),
                    &self.key.as_bytes(),
                    &self.version.as_i32(),
                    &self.algo.to_string(),
                    &**self.inserted_at,
                    &**self.updated_at,
                ],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelQueryCreate { source })
            .and_then(GdprKeyModel::try_from)
    }

    pub async fn delete(
        &self,
        transaction: &Transaction<'_>,
        stmt: &Statement,
    ) -> Result<u64, GdprError> {
        transaction
            .execute(
                stmt,
                &[
                    &self.contact_id.to_string(),
                    &GdprKeyDataGroup::IngestedContact.to_string(),
                ],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelQueryDelete { source })
    }

    pub async fn fetch(
        transaction: &Transaction<'_>,
        stmt: &Statement,
        contact_ids: &[ContactId],
    ) -> Result<Vec<GdprKeyModel>, GdprError> {
        let ids: Vec<String> = contact_ids.iter().map(|i| i.to_string()).collect();
        transaction
            .query(
                stmt,
                &[&ids, &GdprKeyDataGroup::IngestedContact.to_string()],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelQueryFetch { source })
            .and_then(|rows| {
                rows.into_iter()
                    .map(GdprKeyModel::try_from)
                    .collect::<Result<Vec<GdprKeyModel>, GdprError>>()
            })
    }

    pub async fn fetch_one(
        transaction: &Transaction<'_>,
        stmt: &Statement,
        contact_id: &ContactId,
    ) -> Result<Option<GdprKeyModel>, GdprError> {
        transaction
            .query_opt(
                stmt,
                &[
                    &contact_id.to_string(),
                    &GdprKeyDataGroup::IngestedContact.to_string(),
                ],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelQueryFetchOne { source })
            .and_then(|row| row.map(|r| r.try_into()).transpose())
    }

    pub fn new(
        contact_id: Arc<ContactId>,
        data_group: GdprKeyDataGroup,
        version: u32,
    ) -> Result<Self, GdprError> {
        let id = Arc::new(GdprKeyId::new()?);
        let key = Arc::new(GdprKeyKey::new()?);
        let algo = Arc::new(GdprKeyAlgo::Aes256Gcm);
        let version = Arc::new(version.into());
        let now = Arc::new(GdprKeyDateTime::new());

        Ok(GdprKeyModel {
            id,
            contact_id,
            data_group: Arc::new(data_group),
            key,
            version,
            algo,
            inserted_at: now.clone(),
            updated_at: now,
        })
    }

    pub async fn statement_create(transaction: &Transaction<'_>) -> Result<Statement, GdprError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    INSERT INTO {} ( id, contact_id, data_group, key, version, algo, inserted_at, updated_at)
                    VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
                    RETURNING *
                    "#,
                    Settings::get().postgresql.tables.gdpr_keys.table.to_string()
                )
                .as_str(),
                &[
                    Type::VARCHAR,
                    Type::VARCHAR,
                    Type::VARCHAR,
                    Type::BYTEA,
                    Type::INT4,
                    Type::VARCHAR,
                    Type::TIMESTAMP,
                    Type::TIMESTAMP,
                ],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelStatementCreate { source })
    }

    pub async fn statements(
        transaction: &Transaction<'_>,
    ) -> Result<GdprKeyModelStatements, GdprError> {
        let fetch = Self::stmt_fetch(transaction).await?;

        Ok(GdprKeyModelStatements::new(fetch))
    }

    pub async fn stmt_delete_by_contact_id(
        transaction: &Transaction<'_>,
    ) -> Result<Statement, GdprError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    DELETE FROM {}
                    WHERE contact_id = $1
                    AND data_group = $2
                    "#,
                    Settings::get()
                        .postgresql
                        .tables
                        .gdpr_keys
                        .table
                        .to_string()
                )
                .as_str(),
                &[Type::VARCHAR, Type::VARCHAR],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelStatementDeleteByContactId { source })
    }

    pub async fn stmt_fetch_by_contact_id(
        transaction: &Transaction<'_>,
    ) -> Result<Statement, GdprError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    SELECT *
                    FROM {}
                    WHERE contact_id = $1
                    AND data_group = $2
                    "#,
                    Settings::get()
                        .postgresql
                        .tables
                        .gdpr_keys
                        .table
                        .to_string()
                )
                .as_str(),
                &[Type::VARCHAR, Type::VARCHAR],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelStatementFetchByContactId { source })
    }

    async fn stmt_fetch(transaction: &Transaction<'_>) -> Result<Statement, GdprError> {
        transaction
            .prepare_typed(
                format!(
                    r#"
                    SELECT g.*
                    FROM unnest($1) WITH ORDINALITY AS ids_list(id, ord),
                    {} AS g
                    WHERE g.contact_id = ids_list.id
                    AND g.data_group = $2
                    ORDER BY ids_list.ord
                    "#,
                    Settings::get()
                        .postgresql
                        .tables
                        .gdpr_keys
                        .table
                        .to_string()
                )
                .as_str(),
                &[Type::VARCHAR_ARRAY, Type::VARCHAR],
            )
            .await
            .map_err(|source| GdprError::GdprKeyModelStatementFetch { source })
    }
}
