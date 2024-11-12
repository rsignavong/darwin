use crate::entities::{Job, JobCategory, JobDetail, JobType};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "backend", derive(new))]
#[derive(Deserialize, Serialize)]
pub struct RecruiterJobEdited {
    pub job: Job,
    pub detail: JobDetail,
    pub category: JobCategory,
    #[serde(rename = "type")]
    pub type_: JobType,
}
