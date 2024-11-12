use crate::resources::{ProfileLog, ReconciliationCount};
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize, new)]
pub struct ReconciliationDataProfile {
    pub count: Arc<ReconciliationCount>,
    pub log: Arc<ProfileLog>,
}
