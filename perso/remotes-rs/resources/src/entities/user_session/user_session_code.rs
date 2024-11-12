use super::UserSessionError;
use derive_more::Display;
#[cfg(feature = "backend")]
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Display, Eq, PartialEq, Serialize)]
#[display(fmt = "{:06}", "_0")]
pub struct UserSessionCode(u32);

impl TryFrom<&str> for UserSessionCode {
    type Error = UserSessionError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let code = s
            .parse::<u32>()
            .map_err(|_| UserSessionError::InvalidCode(s.to_owned()))?;

        if code > 999_999 {
            return Err(UserSessionError::InvalidCode(s.to_owned()));
        }

        Ok(UserSessionCode(code))
    }
}

impl UserSessionCode {
    #[cfg(feature = "backend")]
    pub fn new() -> Self {
        let mut rng = thread_rng();
        let code: u32 = rng.gen_range(0..1_000_000);
        UserSessionCode(code)
    }
}
