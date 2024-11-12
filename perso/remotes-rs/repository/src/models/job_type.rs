use super::Crud;
use crate::schema::job_types;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable)]
pub struct JobType {
    pub id: Uuid,
    pub type_: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "job_types"]
pub struct JobTypeCommand {
    type_: String,
}

impl Crud<JobTypeCommand> for JobType {
    fn read(conn: &PgConn, job_type_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::job_types::dsl::*;
        Ok(job_types.find(job_type_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, job_type_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::job_types::dsl::*;
        Ok(diesel::delete(job_types.find(job_type_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &JobTypeCommand) -> Result<Self, RepositoryError> {
        use crate::schema::job_types::dsl::*;
        let job_type = diesel::insert_into(job_types)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_type)
    }

    fn update(
        conn: &PgConn,
        job_type_id: &Uuid,
        cmd: &JobTypeCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::job_types::dsl::*;
        let job_type = diesel::update(job_types.find(job_type_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_type)
    }
}
