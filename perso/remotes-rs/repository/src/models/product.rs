use super::Crud;
use crate::schema::products;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct Product {
    id: Uuid,
    name: String,
    description: String,
    price: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "products"]
pub struct ProductCommand {
    name: String,
    description: String,
    price: i32,
}

impl Crud<ProductCommand> for Product {
    fn read(conn: &PgConn, product_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::products::dsl::*;
        Ok(products.find(product_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, product_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::products::dsl::*;
        Ok(diesel::delete(products.find(product_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &ProductCommand) -> Result<Self, RepositoryError> {
        use crate::schema::products::dsl::*;
        let product = diesel::insert_into(products)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(product)
    }

    fn update(
        conn: &PgConn,
        product_id: &Uuid,
        cmd: &ProductCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::products::dsl::*;
        let product = diesel::update(products.find(product_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(product)
    }
}
