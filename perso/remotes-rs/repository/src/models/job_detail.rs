use super::job::Job;
use super::user::User;
use super::Crud;
use crate::schema::job_details;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use derive_new::new;
use diesel::query_dsl::*;
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Job)]
#[belongs_to(User)]
pub struct JobDetail {
    pub id: Uuid,
    pub job_id: Uuid,
    pub user_id: Uuid,
    pub position: String,
    pub description: String,
    pub apply: String,
    pub apply_email: String,
    pub apply_url: Option<String>,
    pub location: Option<String>,
    pub salary: Option<String>,
    pub version: i16,
    pub status: JobDetailStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset, new)]
#[table_name = "job_details"]
pub struct JobDetailCommand {
    job_id: Uuid,
    user_id: Uuid,
    position: String,
    description: String,
    apply: String,
    apply_email: String,
    apply_url: Option<String>,
    location: Option<String>,
    salary: Option<String>,
    version: i16,
    status: JobDetailStatus,
}

#[derive(Debug, DbEnum)]
#[DieselType = "Job_detail_status"]
pub enum JobDetailStatus {
    Draft,
    Reviewed,
    Accepted,
}

impl Crud<JobDetailCommand> for JobDetail {
    fn read(conn: &PgConn, job_detail_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::job_details::dsl::*;
        Ok(job_details
            .find(job_detail_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, job_detail_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::job_details::dsl::*;
        Ok(diesel::delete(job_details.find(job_detail_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &JobDetailCommand) -> Result<Self, RepositoryError> {
        use crate::schema::job_details::dsl::*;
        let job_detail = diesel::insert_into(job_details)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_detail)
    }

    fn update(
        conn: &PgConn,
        job_detail_id: &Uuid,
        cmd: &JobDetailCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::job_details::dsl::*;
        let job_detail = diesel::update(job_details.find(job_detail_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_detail)
    }
}
