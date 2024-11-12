use super::company::Company;
use super::user::User;
use super::Crud;
use crate::schema::purchase_orders;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Company)]
#[belongs_to(User)]
pub struct PurchaseOrder {
    id: Uuid,
    company_id: Uuid,
    user_id: Uuid,
    purchase_order: Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "purchase_orders"]
pub struct PurchaseOrderCommand {
    company_id: Uuid,
    user_id: Uuid,
    purchase_order: Value,
}

impl Crud<PurchaseOrderCommand> for PurchaseOrder {
    fn read(conn: &PgConn, purchase_order_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::purchase_orders::dsl::*;
        Ok(purchase_orders
            .find(purchase_order_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, purchase_order_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::purchase_orders::dsl::*;
        Ok(diesel::delete(purchase_orders.find(purchase_order_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &PurchaseOrderCommand) -> Result<Self, RepositoryError> {
        use crate::schema::purchase_orders::dsl::*;
        let purchase_order_ = diesel::insert_into(purchase_orders)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(purchase_order_)
    }

    fn update(
        conn: &PgConn,
        purchase_order_id: &Uuid,
        cmd: &PurchaseOrderCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::purchase_orders::dsl::*;
        let purchase_order_ = diesel::update(purchase_orders.find(purchase_order_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(purchase_order_)
    }
}
