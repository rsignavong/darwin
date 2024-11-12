use super::company::Company;
use super::Crud;
use crate::schema::users;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use derive_new::new;
use diesel::query_dsl::*;
use diesel::ExpressionMethods;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Company)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub company_id: Option<Uuid>,
}

#[derive(Insertable, AsChangeset, new)]
#[table_name = "users"]
pub struct UserCommand {
    email: String,
    company_id: Option<Uuid>,
}

impl Crud<UserCommand> for User {
    fn read(conn: &PgConn, user_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::users::dsl::*;
        Ok(users.find(user_id.clone()).first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, user_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::users::dsl::*;
        Ok(diesel::delete(users.find(user_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &UserCommand) -> Result<Self, RepositoryError> {
        use crate::schema::users::dsl::*;
        let user = diesel::insert_into(users)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(user)
    }

    fn update(conn: &PgConn, user_id: &Uuid, cmd: &UserCommand) -> Result<Self, RepositoryError> {
        use crate::schema::users::dsl::*;
        let user = diesel::update(users.find(user_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(user)
    }
}

impl User {
    pub fn find_by_email(conn: &PgConn, email_: &str) -> Result<Self, RepositoryError> {
        use crate::schema::users::dsl::*;
        Ok(users.filter(email.eq(email_)).first(conn)?)
    }
}
