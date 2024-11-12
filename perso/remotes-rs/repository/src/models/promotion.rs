use super::product::Product;
use super::Crud;
use crate::schema::promotions;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Product)]
pub struct Promotion {
    id: Uuid,
    product_id: Uuid,
    name: String,
    description: String,
    begin_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    price: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "promotions"]
pub struct PromotionCommand {
    product_id: Uuid,
    name: String,
    description: String,
    begin_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    price: i32,
}

impl Crud<PromotionCommand> for Promotion {
    fn read(conn: &PgConn, promotion_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::promotions::dsl::*;
        Ok(promotions.find(promotion_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, promotion_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::promotions::dsl::*;
        Ok(diesel::delete(promotions.find(promotion_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &PromotionCommand) -> Result<Self, RepositoryError> {
        use crate::schema::promotions::dsl::*;
        let promotion = diesel::insert_into(promotions)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(promotion)
    }

    fn update(
        conn: &PgConn,
        promotion_id: &Uuid,
        cmd: &PromotionCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::promotions::dsl::*;
        let promotion = diesel::update(promotions.find(promotion_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(promotion)
    }
}
