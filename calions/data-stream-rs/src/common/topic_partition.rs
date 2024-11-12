use ahash::AHasher;
use derive_more::Display;
use derive_new::new;
use rdkafka::consumer::{base_consumer::BaseConsumer, Consumer};
use rdkafka::error::KafkaError;
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Deserialize, Display, Eq, Hash, PartialEq, Serialize, new)]
#[display(fmt = "{}-{}", _0, _1)]
pub struct TopicPartition(String, i32);

impl TopicPartition {
    pub fn from_key(key: &str, nb_partition: usize, topic: &str) -> Self {
        let mut hasher = AHasher::default();
        key.hash(&mut hasher);
        let partition = (hasher.finish() % nb_partition as u64) as i32;
        TopicPartition(topic.to_owned(), partition)
    }

    pub fn inner(&self) -> (String, i32) {
        (self.0.to_owned(), self.1)
    }

    pub fn partition(&self) -> i32 {
        self.1
    }

    pub fn topic(&self) -> &str {
        &self.0
    }

    pub fn topic_partition_list(
        base_consumer: &BaseConsumer,
        topic: &str,
    ) -> Result<Vec<Self>, KafkaError> {
        let mut topic_partition_list = Vec::new();
        let metadata = base_consumer.fetch_metadata(Some(topic), None)?;
        for meta_topic in metadata.topics() {
            for partition in meta_topic.partitions() {
                topic_partition_list
                    .push(TopicPartition(meta_topic.name().to_owned(), partition.id()));
            }
        }

        Ok(topic_partition_list)
    }
}
