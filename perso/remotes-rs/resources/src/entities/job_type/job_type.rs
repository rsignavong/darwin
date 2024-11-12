use super::{JobTypeId, JobTypeTitle};
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use repository::job_type::JobType as JobTypeRepository;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JobType {
    pub id: JobTypeId,
    pub title: JobTypeTitle,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "backend")]
impl From<JobTypeRepository> for JobType {
    fn from(job_type: JobTypeRepository) -> Self {
        JobType {
            id: job_type.id.into(),
            title: job_type.type_.into(),
            created_at: job_type.created_at,
            updated_at: job_type.updated_at,
        }
    }
}
