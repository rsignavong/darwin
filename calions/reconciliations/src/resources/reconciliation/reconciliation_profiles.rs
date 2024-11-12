use super::ReconciliationKey;
use crate::resources::ProfileId;
use ahash::AHashMap;

pub type ReconciliationProfiles = AHashMap<ReconciliationKey, ProfileId>;
