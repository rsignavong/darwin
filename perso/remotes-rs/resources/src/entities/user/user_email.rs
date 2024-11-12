use super::UserError;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use validator::validate_email;

#[derive(Debug, Deref, Deserialize, Serialize)]
pub struct UserEmail(String);

impl TryFrom<&str> for UserEmail {
    type Error = UserError;

    fn try_from(email: &str) -> Result<Self, Self::Error> {
        if !validate_email(email) {
            return Err(UserError::InvalidEmail(email.to_owned()));
        }

        Ok(UserEmail(email.to_owned()))
    }
}
