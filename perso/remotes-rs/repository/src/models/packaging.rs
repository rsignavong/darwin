use super::feature::Feature;
use super::product::Product;
use super::Crud;
use crate::schema::packagings;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Product)]
#[belongs_to(Feature)]
pub struct Packaging {
    id: Uuid,
    product_id: Uuid,
    feature_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "packagings"]
pub struct PackagingCommand {
    product_id: Uuid,
    feature_id: Uuid,
}

impl Crud<PackagingCommand> for Packaging {
    fn read(conn: &PgConn, packaging_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::packagings::dsl::*;
        Ok(packagings.find(packaging_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, packaging_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::packagings::dsl::*;
        Ok(diesel::delete(packagings.find(packaging_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &PackagingCommand) -> Result<Self, RepositoryError> {
        use crate::schema::packagings::dsl::*;
        let packaging = diesel::insert_into(packagings)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(packaging)
    }

    fn update(
        conn: &PgConn,
        packaging_id: &Uuid,
        cmd: &PackagingCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::packagings::dsl::*;
        let packaging = diesel::update(packagings.find(packaging_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(packaging)
    }
}
