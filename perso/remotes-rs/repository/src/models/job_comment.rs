use super::job::Job;
use super::user::User;
use super::Crud;
use crate::schema::job_comments;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Job)]
#[belongs_to(User)]
pub struct JobComment {
    id: Uuid,
    job_id: Uuid,
    user_id: Uuid,
    #[column_name = "comment"]
    message: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "job_comments"]
pub struct JobCommentCommand {
    job_id: Uuid,
    user_id: Uuid,
    #[column_name = "comment"]
    message: String,
}

impl Crud<JobCommentCommand> for JobComment {
    fn read(conn: &PgConn, job_comment_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::job_comments::dsl::*;
        Ok(job_comments
            .find(job_comment_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, job_comment_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::job_comments::dsl::*;
        Ok(diesel::delete(job_comments.find(job_comment_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &JobCommentCommand) -> Result<Self, RepositoryError> {
        use crate::schema::job_comments::dsl::*;
        let job_comment = diesel::insert_into(job_comments)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_comment)
    }

    fn update(
        conn: &PgConn,
        job_comment_id: &Uuid,
        cmd: &JobCommentCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::job_comments::dsl::*;
        let job_comment = diesel::update(job_comments.find(job_comment_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_comment)
    }
}
