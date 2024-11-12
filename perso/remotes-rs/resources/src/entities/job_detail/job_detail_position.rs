use super::JobDetailError;
use derive_more::Deref;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use validator::validate_length;

#[derive(Debug, Deref, Deserialize, Serialize, new)]
#[serde(try_from = "String")]
pub struct JobDetailPosition(String);

impl TryFrom<String> for JobDetailPosition {
    type Error = JobDetailError;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        if !validate_length(&s, Some(3), Some(80), None) {
            return Err(JobDetailError::InvalidPositionLength(s));
        }

        Ok(JobDetailPosition(s))
    }
}
