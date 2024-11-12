use super::job::Job;
use super::job_detail::JobDetail;
use super::Crud;
use crate::schema::job_listings;
use crate::{PgConn, RepositoryError};
use chrono::{DateTime, Utc};
use diesel::dsl::*;
use diesel::query_dsl::*;
use serde_json::Value;
use uuid::Uuid;

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Job)]
#[belongs_to(JobDetail)]
pub struct JobListing {
    id: Uuid,
    job_id: Uuid,
    job_detail_id: Uuid,
    display: Value,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "job_listings"]
pub struct JobListingCommand {
    job_id: Uuid,
    job_detail_id: Uuid,
    display: Value,
}

impl Crud<JobListingCommand> for JobListing {
    fn read(conn: &PgConn, job_listing_id: &Uuid) -> Result<Self, RepositoryError> {
        use crate::schema::job_listings::dsl::*;
        Ok(job_listings
            .find(job_listing_id.clone())
            .first::<Self>(conn)?)
    }

    fn delete(conn: &PgConn, job_listing_id: &Uuid) -> Result<usize, RepositoryError> {
        use crate::schema::job_listings::dsl::*;
        Ok(diesel::delete(job_listings.find(job_listing_id.clone())).execute(conn)?)
    }

    fn create(conn: &PgConn, cmd: &JobListingCommand) -> Result<Self, RepositoryError> {
        use crate::schema::job_listings::dsl::*;
        let job_listing = insert_into(job_listings)
            .values(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_listing)
    }

    fn update(
        conn: &PgConn,
        job_listing_id: &Uuid,
        cmd: &JobListingCommand,
    ) -> Result<Self, RepositoryError> {
        use crate::schema::job_listings::dsl::*;
        let job_listing = diesel::update(job_listings.find(job_listing_id))
            .set(cmd)
            .get_result::<Self>(conn)?;
        Ok(job_listing)
    }
}
