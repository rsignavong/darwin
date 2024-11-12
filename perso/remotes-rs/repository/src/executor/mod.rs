use super::{PgConn, RepositoryError};
use async_trait::async_trait;
use std::convert::{TryFrom, TryInto};
use tokio_postgres::{types::ToSql, Row};

pub mod advertiser_post_creator;

#[async_trait]
pub trait Executor {
    async fn execute(
        conn: &PgConn,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<u64, RepositoryError>
    where
        Self: Sized + TryFrom<Row, Error = RepositoryError>,
    {
        let stmt = conn.prepare_cached(query).await?;
        let res = conn.execute(&stmt, params).await?;

        Ok(res)
    }

    async fn query_one(
        conn: &PgConn,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Self, RepositoryError>
    where
        Self: Sized + TryFrom<Row, Error = RepositoryError>,
    {
        let stmt = conn.prepare_cached(query).await?;
        let res = conn.query_one(&stmt, params).await?;

        res.try_into()
    }

    async fn query(
        conn: &PgConn,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Self>, RepositoryError>
    where
        Self: Sized + TryFrom<Row, Error = RepositoryError>,
    {
        let stmt = conn.prepare_cached(query).await?;
        let rows = conn.query(&stmt, params).await?;

        let res = rows
            .iter()
            .map(|r| r.try_into())
            .collect::<Result<Vec<Self>, RepositoryError>>()?;
        Ok(res)
    }
}
