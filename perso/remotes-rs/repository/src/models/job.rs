use super::company::Company;
use super::Crud;
use crate::schema::jobs;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use derive_new::new;
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Company)]
pub struct Job {
    pub id: Uuid,
    pub company_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[column_name = "job_type_id"]
    pub type_id: Uuid,
    #[column_name = "job_category_id"]
    pub category_id: Uuid,
}

#[derive(Insertable, AsChangeset, new)]
#[table_name = "jobs"]
pub struct JobCommand {
    #[column_name = "job_category_id"]
    category_id: Uuid,
    #[column_name = "job_type_id"]
    type_id: Uuid,
    company_id: Option<Uuid>,
}

impl Crud<JobCommand> for Job {
    fn read(conn: &PgConn, job_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::jobs::dsl::*;
        Ok(jobs.find(job_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, job_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::jobs::dsl::*;
        Ok(diesel::delete(jobs.find(job_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &JobCommand) -> Result<Self, RepositoryError> {
        use crate::schema::jobs::dsl::*;
        let job = diesel::insert_into(jobs)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(job)
    }

    fn update(conn: &PgConn, job_id: &Uuid, cmd: &JobCommand) -> Result<Self, RepositoryError> {
        use crate::schema::jobs::dsl::*;
        let job = diesel::update(jobs.find(job_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(job)
    }
}
