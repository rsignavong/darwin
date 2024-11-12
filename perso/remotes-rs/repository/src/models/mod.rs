use super::{PgConn, RepositoryError};
use uuid::Uuid;

pub mod company;
pub mod credit;
pub mod feature;
pub mod feedback;
pub mod job;
pub mod job_category;
pub mod job_comment;
pub mod job_detail;
pub mod job_listing;
pub mod job_type;
pub mod packaging;
pub mod payment;
pub mod product;
pub mod promotion;
pub mod purchase_order;
pub mod tag;
pub mod tagged_job;
pub mod user;
pub mod user_account;
pub mod user_session;

pub trait Crud<T> {
    fn create(conn: &PgConn, cmd: &T) -> Result<Self, RepositoryError>
    where
        Self: Sized;
    fn read(conn: &PgConn, id: &Uuid) -> Result<Self, RepositoryError>
    where
        Self: Sized;
    fn update(conn: &PgConn, id: &Uuid, cmd: &T) -> Result<Self, RepositoryError>
    where
        Self: Sized;
    fn upsert(_conn: &PgConn, _cmd: &T) -> Result<Self, RepositoryError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
    fn delete(_conn: &PgConn, _id: &Uuid) -> Result<usize, RepositoryError>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}
