use super::{JobCategoryId, JobCategoryTitle};
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use repository::job_category::JobCategory as JobCategoryRepository;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct JobCategory {
    pub id: JobCategoryId,
    pub title: JobCategoryTitle,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "backend")]
impl From<JobCategoryRepository> for JobCategory {
    fn from(job_category: JobCategoryRepository) -> Self {
        JobCategory {
            id: job_category.id.into(),
            title: job_category.category.into(),
            created_at: job_category.created_at,
            updated_at: job_category.updated_at,
        }
    }
}
