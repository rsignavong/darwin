use super::job::Job;
use super::tag::Tag;
use super::Crud;
use crate::schema::tagged_jobs;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Job)]
#[belongs_to(Tag)]
pub struct TaggedJob {
    id: Uuid,
    job_id: Uuid,
    tag_id: Uuid,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "tagged_jobs"]
pub struct TaggedJobCommand {
    job_id: Uuid,
    tag_id: Uuid,
}

impl Crud<TaggedJobCommand> for TaggedJob {
    fn read(conn: &PgConn, tagged_job_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::tagged_jobs::dsl::*;
        Ok(tagged_jobs
            .find(tagged_job_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, tagged_job_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::tagged_jobs::dsl::*;
        Ok(diesel::delete(tagged_jobs.find(tagged_job_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &TaggedJobCommand) -> Result<Self, RepositoryError> {
        use crate::schema::tagged_jobs::dsl::*;
        let tagged_job = diesel::insert_into(tagged_jobs)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(tagged_job)
    }

    fn update(
        conn: &PgConn,
        tagged_job_id: &Uuid,
        cmd: &TaggedJobCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::tagged_jobs::dsl::*;
        let tagged_job = diesel::update(tagged_jobs.find(tagged_job_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(tagged_job)
    }
}
