use super::user::User;
use super::Crud;
use crate::schema::user_accounts;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::query_dsl::*;
use diesel_derive_enum::DbEnum;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(User)]
pub struct UserAccount {
    pub id: Uuid,
    pub user_id: Uuid,
    pub status: UserAccountStatus,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "user_accounts"]
pub struct UserAccountCommand {
    user_id: Uuid,
    status: UserAccountStatus,
    comment: Option<String>,
}

#[derive(Debug, DbEnum)]
#[DieselType = "User_account_status"]
pub enum UserAccountStatus {
    Invalid,
    Valid,
}

impl Crud<UserAccountCommand> for UserAccount {
    fn read(conn: &PgConn, user_account_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::user_accounts::dsl::*;
        Ok(user_accounts
            .find(user_account_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, user_account_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::user_accounts::dsl::*;
        Ok(diesel::delete(user_accounts.find(user_account_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &UserAccountCommand) -> Result<Self, RepositoryError> {
        use crate::schema::user_accounts::dsl::*;
        let user_account = diesel::insert_into(user_accounts)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(user_account)
    }

    fn update(
        conn: &PgConn,
        user_account_id: &Uuid,
        cmd: &UserAccountCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::user_accounts::dsl::*;
        let user_account = diesel::update(user_accounts.find(user_account_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(user_account)
    }
}
