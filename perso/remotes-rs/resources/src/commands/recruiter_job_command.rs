use crate::entities::UserEmail;
use crate::entities::{JobCategoryId, JobTypeId};
use crate::entities::{JobDetailApply, JobDetailApplyEmail, JobDetailApplyUrl};
use crate::entities::{
    JobDetailDescription, JobDetailLocation, JobDetailPosition, JobDetailSalary,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct AdvertiserPostCommand {
    pub user_email: Option<UserEmail>,
    pub post_details: PostDetails,
    // pub category_id: JobCategoryId,
    // pub type_id: JobTypeId,
    // pub position: JobDetailPosition,
    // pub description: JobDetailDescription,
    // pub apply: JobDetailApply,
    // pub apply_email: JobDetailApplyEmail,
    // pub apply_url: Option<JobDetailApplyUrl>,
    // pub location: Option<JobDetailLocation>,
    // pub salary: Option<JobDetailSalary>,
}

impl RecruiterJobCommand {
    #[cfg(feature = "backend")]
    pub fn apply_url_to_string(&self) -> Option<String> {
        self.apply_url.as_ref().map(|u| u.to_string())
    }

    #[cfg(feature = "backend")]
    pub fn location_to_string(&self) -> Option<String> {
        self.location.as_ref().map(|l| l.to_string())
    }

    #[cfg(feature = "backend")]
    pub fn salary_to_string(&self) -> Option<String> {
        self.salary.as_ref().map(|s| s.to_string())
    }
}
