use super::purchase_order::PurchaseOrder;
use super::Crud;
use crate::schema::payments;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(PurchaseOrder)]
pub struct Payment {
    id: Uuid,
    purchase_order_id: Uuid,
    metadata: Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "payments"]
pub struct PaymentCommand {
    purchase_order_id: Uuid,
    metadata: Value,
}

impl Crud<PaymentCommand> for Payment {
    fn read(conn: &PgConn, payment_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::payments::dsl::*;
        Ok(payments.find(payment_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, payment_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::payments::dsl::*;
        Ok(diesel::delete(payments.find(payment_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &PaymentCommand) -> Result<Self, RepositoryError> {
        use crate::schema::payments::dsl::*;
        let payment = diesel::insert_into(payments)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(payment)
    }

    fn update(
        conn: &PgConn,
        payment_id: &Uuid,
        cmd: &PaymentCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::payments::dsl::*;
        let payment = diesel::update(payments.find(payment_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(payment)
    }
}
