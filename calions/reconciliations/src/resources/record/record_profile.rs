use crate::resources::{Profile, ProfileId, ResourcesError};
use derive_more::Display;
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Debug, Display, Serialize)]
#[serde(untagged)]
pub enum RecordProfile {
    Created(ProfileId),
    Updated(ProfileId),
}

impl TryFrom<Profile> for RecordProfile {
    type Error = ResourcesError;

    fn try_from(profile: Profile) -> Result<Self, Self::Error> {
        let record_profile = match profile {
            Profile::Old(profile_id) => RecordProfile::Updated(profile_id),
            Profile::New => RecordProfile::Created(ProfileId::new()?),
        };

        Ok(record_profile)
    }
}

impl RecordProfile {
    pub fn unwrap(&self) -> &ProfileId {
        match self {
            RecordProfile::Created(profile_id) | RecordProfile::Updated(profile_id) => profile_id,
        }
    }
}
