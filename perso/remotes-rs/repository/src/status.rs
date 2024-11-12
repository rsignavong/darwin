use crate::{PgConn, RepositoryError};

pub struct Status;

impl Status {
    pub fn get(conn: &PgConn) -> Result<usize, RepositoryError> {
        Ok(diesel::sql_query("SELECT 1").execute(conn)?)
    }
}
