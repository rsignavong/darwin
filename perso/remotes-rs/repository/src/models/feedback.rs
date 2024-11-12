use super::company::Company;
use super::user::User;
use super::Crud;
use crate::schema::feedbacks;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Company)]
#[belongs_to(User)]
pub struct Feedback {
    id: Uuid,
    company_id: Uuid,
    user_id: Uuid,
    feedback: String,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "feedbacks"]
pub struct FeedbackCommand {
    company_id: Uuid,
    user_id: Uuid,
    feedback: String,
}

impl Crud<FeedbackCommand> for Feedback {
    fn read(conn: &PgConn, feedback_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::feedbacks::dsl::*;
        Ok(feedbacks.find(feedback_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, feedback_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::feedbacks::dsl::*;
        Ok(diesel::delete(feedbacks.find(feedback_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &FeedbackCommand) -> Result<Self, RepositoryError> {
        use crate::schema::feedbacks::dsl::*;
        let feedback_ = diesel::insert_into(feedbacks)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(feedback_)
    }

    fn update(
        conn: &PgConn,
        feedback_id: &Uuid,
        cmd: &FeedbackCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::feedbacks::dsl::*;
        let feedback_ = diesel::update(feedbacks.find(feedback_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(feedback_)
    }
}
