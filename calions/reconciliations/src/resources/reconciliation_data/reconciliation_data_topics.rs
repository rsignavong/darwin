use crate::resources::ProcessorTopic;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Debug, Deserialize, Serialize, new)]
pub struct ReconciliationDataTopics {
    #[serde(alias = "gdpr_data_anonymization_request_validations_topics")]
    pub anonymizations_topics: Arc<Vec<ProcessorTopic>>,
    pub raw_records_topics: Arc<Vec<ProcessorTopic>>,
    pub reconciliations_records_topic: Arc<ProcessorTopic>,
}
