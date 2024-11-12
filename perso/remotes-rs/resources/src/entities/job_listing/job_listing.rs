use super::JobListingId;
use crate::entities::{JobDetailId, JobId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JobListing {
    id: JobListingId,
    job_id: JobId,
    job_detail_id: JobDetailId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
