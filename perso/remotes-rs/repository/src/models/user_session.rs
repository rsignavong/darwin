use super::user_account::UserAccount;
use super::Crud;
use crate::schema::user_sessions;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use diesel::ExpressionMethods;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(UserAccount)]
pub struct UserSession {
    pub id: Uuid,
    pub user_account_id: Uuid,
    pub code: i32,
    pub token: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "user_sessions"]
pub struct UserSessionCommand {
    user_account_id: Uuid,
    code: i32,
    token: Option<String>,
}

impl Crud<UserSessionCommand> for UserSession {
    fn read(conn: &PgConn, user_session_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::user_sessions::dsl::*;
        Ok(user_sessions
            .find(user_session_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, user_session_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::user_sessions::dsl::*;
        Ok(diesel::delete(user_sessions.find(user_session_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &UserSessionCommand) -> Result<Self, RepositoryError> {
        use crate::schema::user_sessions::dsl::*;
        let user_session = diesel::insert_into(user_sessions)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(user_session)
    }

    fn update(
        conn: &PgConn,
        user_session_id: &Uuid,
        cmd: &UserSessionCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::user_sessions::dsl::*;
        let user_session = diesel::update(user_sessions.find(user_session_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(user_session)
    }
}

impl UserSession {
    pub fn find_by_token(conn: &PgConn, token_: &str) -> Result<Self, RepositoryError> {
        use crate::schema::user_sessions::dsl::*;
        Ok(user_sessions.filter(token.eq(token_)).first(conn)?)
    }
}
