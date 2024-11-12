use crate::stream::{OutputStreamError, OutputStreamKeyError};
use anyhow::Error as AnyError;
use rayon::ThreadPoolBuildError;
use rdkafka::error::KafkaError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProducerError {
    #[error("ProducerConfig")]
    ProducerConfig(#[from] ProducerConfigError),
    #[error("ProducerCreateClientConsumer")]
    ProducerCreateClientConsumer { source: KafkaError },
    #[error("ProducerCreateClientProducer")]
    ProducerCreateClientProducer { source: KafkaError },
    #[error("ProducerJsonSerializationUnsupported")]
    ProducerJsonSerializationUnsupported,
    #[error("ProducerOutputStream")]
    ProducerOutputStream(#[from] OutputStreamError),
    #[error("ProducerSendRecord: {msg}")]
    ProducerSendRecord { source: KafkaError, msg: String },
    #[error("ProducerStreamKey")]
    ProducerStreamKey(#[from] OutputStreamKeyError),
    #[error("ProducerThreadPoolBuild")]
    ProducerThreadPoolBuild(#[from] ThreadPoolBuildError),
    #[error("ProducerTopicPartitionMetadata")]
    ProducerTopicPartitionMetadata { source: KafkaError },
}

#[derive(Debug, Error)]
pub enum ProducerConfigError {
    #[error("ProducerConfigAny")]
    ProducerConfigAny(#[from] AnyError),
    #[error("ProducerConfigBuilder")]
    ProducerConfigBuilder { source: AnyError },
}
