use super::GdprError;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Display, Serialize)]
pub enum GdprKeyAlgo {
    #[display(fmt = "Aes256Gcm")]
    Aes256Gcm,
}

impl TryFrom<&str> for GdprKeyAlgo {
    type Error = GdprError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "Aes256Gcm" => Ok(GdprKeyAlgo::Aes256Gcm),
            other => Err(GdprError::GdprKeyAlgoTryFromStr(other.to_owned())),
        }
    }
}
