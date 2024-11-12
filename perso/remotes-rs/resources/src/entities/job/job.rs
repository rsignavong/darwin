use super::JobId;
use crate::entities::{CompanyId, JobCategoryId, JobTypeId};
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use repository::job::Job as JobRepository;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Job {
    pub id: JobId,
    pub category_id: JobCategoryId,
    pub type_id: JobTypeId,
    pub company_id: Option<CompanyId>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "backend")]
impl From<JobRepository> for Job {
    fn from(job: JobRepository) -> Self {
        Job {
            id: job.id.into(),
            category_id: job.category_id.into(),
            type_id: job.type_id.into(),
            company_id: job.company_id.as_ref().map(|c| c.clone().into()),
            created_at: job.created_at,
            updated_at: job.updated_at,
        }
    }
}
