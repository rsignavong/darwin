use super::GdprError;
use derive_more::Display;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Deserialize, Display, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum GdprKeyDataGroup {
    #[display(fmt = "ingested-contact")]
    IngestedContact,
}

impl TryFrom<&str> for GdprKeyDataGroup {
    type Error = GdprError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "ingested-contact" => Ok(GdprKeyDataGroup::IngestedContact),
            other => Err(GdprError::GdprKeyDataGroupTryFromStr(other.to_owned())),
        }
    }
}
