use super::ProfileId;
use crate::resources::ReconciliationKey;
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, new)]
pub struct ProfileLog {
    pub key: ReconciliationKey,
    pub old: Option<ProfileId>,
    pub new: ProfileId,
}
