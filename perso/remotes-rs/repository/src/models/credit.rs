use super::feature::Feature;
use super::job::Job;
use super::purchase_order::PurchaseOrder;
use super::Crud;
use crate::schema::credits;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(PurchaseOrder)]
#[belongs_to(Feature)]
#[belongs_to(Job)]
pub struct Credit {
    id: Uuid,
    purchase_order_id: Uuid,
    feature_id: Uuid,
    job_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "credits"]
pub struct CreditCommand {
    purchase_order_id: Uuid,
    feature_id: Uuid,
    job_id: Uuid,
}

impl Crud<CreditCommand> for Credit {
    fn read(conn: &PgConn, credit_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::credits::dsl::*;
        Ok(credits.find(credit_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, credit_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::credits::dsl::*;
        Ok(diesel::delete(credits.find(credit_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &CreditCommand) -> Result<Self, RepositoryError> {
        use crate::schema::credits::dsl::*;
        let credit = diesel::insert_into(credits)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(credit)
    }

    fn update(
        conn: &PgConn,
        credit_id: &Uuid,
        cmd: &CreditCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::credits::dsl::*;
        let credit = diesel::update(credits.find(credit_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(credit)
    }
}
