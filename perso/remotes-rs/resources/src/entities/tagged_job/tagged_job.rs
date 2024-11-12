use crate::entities::{JobId, TagId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaggedJob {
    job_id: JobId,
    tag_id: TagId,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}
