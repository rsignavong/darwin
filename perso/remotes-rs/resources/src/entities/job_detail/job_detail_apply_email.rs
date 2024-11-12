use super::JobDetailError;
use derive_more::{Deref, From};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use validator::validate_email;

#[derive(Debug, Deref, Deserialize, From, Serialize)]
pub struct JobDetailApplyEmail(String);

impl TryFrom<&str> for JobDetailApplyEmail {
    type Error = JobDetailError;

    fn try_from(email: &str) -> Result<Self, Self::Error> {
        if !validate_email(email) {
            return Err(JobDetailError::InvalidApplyEmail(email.to_owned()));
        }

        Ok(JobDetailApplyEmail(email.to_owned()))
    }
}
