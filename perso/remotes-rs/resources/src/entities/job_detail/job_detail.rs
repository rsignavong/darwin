use super::JobDetailError;
use super::{JobDetailApply, JobDetailApplyEmail, JobDetailApplyUrl};
use super::{JobDetailDescription, JobDetailLocation, JobDetailPosition, JobDetailSalary};
use super::{JobDetailId, JobDetailStatus, JobDetailVersion};
use crate::entities::{JobId, UserId};
use chrono::{DateTime, Utc};
#[cfg(feature = "backend")]
use repository::job_detail::JobDetail as JobDetailRepository;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};

#[derive(Deserialize, Serialize)]
pub struct JobDetail {
    pub id: JobDetailId,
    pub job_id: JobId,
    pub user_id: UserId,
    pub position: JobDetailPosition,
    pub description: JobDetailDescription,
    pub apply: JobDetailApply,
    pub apply_email: JobDetailApplyEmail,
    pub apply_url: Option<JobDetailApplyUrl>,
    pub location: Option<JobDetailLocation>,
    pub salary: Option<JobDetailSalary>,
    pub version: JobDetailVersion,
    pub status: JobDetailStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[cfg(feature = "backend")]
impl TryFrom<JobDetailRepository> for JobDetail {
    type Error = JobDetailError;

    fn try_from(job_detail: JobDetailRepository) -> Result<Self, Self::Error> {
        Ok(JobDetail {
            id: job_detail.id.into(),
            job_id: job_detail.job_id.into(),
            user_id: job_detail.user_id.into(),
            position: job_detail.position.try_into()?,
            description: job_detail.description.into(),
            apply: job_detail.apply.into(),
            apply_email: job_detail.apply_email.into(),
            apply_url: job_detail.apply_url.as_ref().map(|u| u.clone().into()),
            location: job_detail.location.as_ref().map(|l| l.clone().into()),
            salary: job_detail.salary.as_ref().map(|s| s.clone().into()),
            version: job_detail.version.into(),
            status: job_detail.status.into(),
            created_at: job_detail.created_at.into(),
            updated_at: job_detail.updated_at.into(),
        })
    }
}
