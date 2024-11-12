#[cfg(feature = "backend")]
use repository::job_detail::JobDetailStatus as JobDetailStatusRepository;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum JobDetailStatus {
    Draft,
    Reviewed,
    Accepted,
}

#[cfg(feature = "backend")]
impl From<JobDetailStatusRepository> for JobDetailStatus {
    fn from(job_detail_status: JobDetailStatusRepository) -> Self {
        match job_detail_status {
            JobDetailStatusRepository::Draft => Self::Draft,
            JobDetailStatusRepository::Reviewed => Self::Reviewed,
            JobDetailStatusRepository::Accepted => Self::Accepted,
        }
    }
}
