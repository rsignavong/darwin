use super::ReconciliationDataTopics;
use super::{ReconciliationDataContext, ReconciliationDataMapping, ReconciliationDataProfile};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data", rename_all = "snake_case")]
pub enum ReconciliationData {
    Context(ReconciliationDataContext),
    Mapping(ReconciliationDataMapping),
    Profile(ReconciliationDataProfile),
    Topics(ReconciliationDataTopics),
}
