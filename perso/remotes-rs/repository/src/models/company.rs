use super::Crud;
use crate::schema::companies;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
#[table_name = "companies"]
pub struct Company {
    id: Uuid,
    name: String,
    description: String,
    logo_url: Option<String>,
    tag_line: Option<String>,
    website_url: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "companies"]
pub struct CompanyCommand {
    name: String,
    description: String,
    logo_url: Option<String>,
    tag_line: Option<String>,
    website_url: Option<String>,
}

impl Crud<CompanyCommand> for Company {
    fn read(conn: &PgConn, company_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::companies::dsl::*;
        Ok(companies.find(company_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, company_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::companies::dsl::*;
        Ok(diesel::delete(companies.find(company_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &CompanyCommand) -> Result<Self, RepositoryError> {
        use crate::schema::companies::dsl::*;
        let company = diesel::insert_into(companies)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(company)
    }

    fn update(
        conn: &PgConn,
        company_id: &Uuid,
        cmd: &CompanyCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::companies::dsl::*;
        let company = diesel::update(companies.find(company_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(company)
    }
}
