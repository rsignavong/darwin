use super::JobListingError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct JobListingId(Uuid);

impl TryFrom<&str> for JobListingId {
    type Error = JobListingError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Ok(JobListingId(Uuid::parse_str(s)?))
    }
}

impl JobListingId {
    pub fn new() -> Self {
        JobListingId(Uuid::new_v4())
    }
}
