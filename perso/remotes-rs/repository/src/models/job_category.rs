use super::Crud;
use crate::schema::job_categories;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
#[table_name = "job_categories"]
pub struct JobCategory {
    pub id: Uuid,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "job_categories"]
pub struct JobCategoryCommand {
    category: String,
}

impl Crud<JobCategoryCommand> for JobCategory {
    fn read(conn: &PgConn, job_category_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::job_categories::dsl::*;
        Ok(job_categories
            .find(job_category_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, job_category_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::job_categories::dsl::*;
        Ok(diesel::delete(job_categories.find(job_category_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &JobCategoryCommand) -> Result<Self, RepositoryError> {
        use crate::schema::job_categories::dsl::*;
        let job_categroy = diesel::insert_into(job_categories)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_categroy)
    }

    fn update(
        conn: &PgConn,
        job_category_id: &Uuid,
        cmd: &JobCategoryCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::job_categories::dsl::*;
        let job_categroy = diesel::update(job_categories.find(job_category_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_categroy)
    }
}
