use crate::processor::ProcessorError;
use anyhow::Error as AnyError;
use rayon::ThreadPoolBuildError;
use rdkafka::error::KafkaError;
use serde_json::Error as SerdeJsonError;
use std::io::Error as IoError;
use std::str::Utf8Error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConsumerError {
    #[error("ConsumerBaseConsumerCreate")]
    ConsumerBaseConsumerCreate { source: KafkaError },
    #[error("ConsumerBorrowedMessage")]
    ConsumerBorrowedMessage { source: KafkaError },
    #[error("ConsumerCommitMessage")]
    ConsumerCommitMessage { source: KafkaError },
    #[error("ConsumerConfig")]
    ConsumerConfig(#[from] ConsumerConfigError),
    #[error("ConsumerConsumeStream")]
    ConsumerConsumeStream { source: AnyError },
    #[error("ConsumerCreateClientProducer")]
    ConsumerCreateClientProducer { source: KafkaError },
    #[error("ConsumerCreateDir")]
    ConsumerCreateDir { source: IoError },
    #[error("ConsumerCreateOpenFile")]
    ConsumerCreateOpenFile { source: IoError },
    #[error("ConsumerDebeziumDeserializeEvent: {0}")]
    ConsumerDebeziumDeserializeEvent(String),
    #[error("ConsumerDeserializeProcessingTime: {0}")]
    ConsumerDeserializeProcessingTime(i64),
    #[error("ConsumerFetchWatermarks")]
    ConsumerFetchWatermarks { source: KafkaError },
    #[error("ConsumerFlushWriter")]
    ConsumerFlushWriter { source: IoError },
    #[error("ConsumerMapFile")]
    ConsumerMapFile { source: IoError },
    #[error("ConsumerOffsetProducerMissing")]
    ConsumerOffsetProducerMissing,
    #[error("ConsumerPartitionQueueUnsplitted")]
    ConsumerPartitionQueueUnsplitted,
    #[error("ConsumerProcessorDispatch")]
    ConsumerProcessorDispatch { source: ProcessorError },
    #[error("ConsumerProcessorNotify")]
    ConsumerProcessorNotify { source: ProcessorError },
    #[error("ConsumerProcessorSender")]
    ConsumerProcessorSender { source: AnyError },
    #[error("ConsumerReadIndex")]
    ConsumerReadIndex { source: IoError },
    #[error("ConsumerReadFile")]
    ConsumerReadFile { source: IoError },
    #[error("ConsumerSendOffset: {msg}")]
    ConsumerSendOffset { source: KafkaError, msg: String },
    #[error("ConsumerSerializeDataStreamOffset")]
    ConsumerSerializeDataStreamOffset { source: SerdeJsonError },
    #[error("ConsumerSetFileLength")]
    ConsumerSetFileLength { source: IoError },
    #[error("ConsumerSplitPartitionQueue")]
    ConsumerSplitPartitionQueue,
    #[error("ConsumerStream")]
    ConsumerStream(#[from] ConsumerStreamError),
    #[error("ConsumerStreamDecode")]
    ConsumerStreamDecode { source: Utf8Error },
    #[error("ConsumerStreamNoBytes")]
    ConsumerStreamNoBytes,
    #[error("ConsumerStreamPayload")]
    ConsumerStreamPayload { source: SerdeJsonError },
    #[error("ConsumerThreadPoolBuild")]
    ConsumerThreadPoolBuild(#[from] ThreadPoolBuildError),
    #[error("ConsumerTopicPartitionAssignation")]
    ConsumerTopicPartitionAssignation { source: KafkaError },
    #[error("ConsumerTopicPartitionEmpty")]
    ConsumerTopicPartitionEmpty,
    #[error("ConsumerTopicPartitionMetadata")]
    ConsumerTopicPartitionMetadata { source: KafkaError },
    #[error("ConsumerWriteIndex")]
    ConsumerWriteIndex { source: IoError },
}

#[derive(Debug, Error)]
pub enum ConsumerConfigError {
    #[error("ConsumerConfigAny")]
    ConsumerConfigAny(#[from] AnyError),
    #[error("ConsumerConfigBuilder")]
    ConsumerConfigBuilder { source: AnyError },
    #[error("ConsumerConfigOffsetStorageTopicEmpty")]
    ConsumerConfigOffsetStorageTopicEmpty,
}

#[derive(Debug, Error)]
pub enum ConsumerStreamError {
    #[error("ConsumerStreamAny")]
    ConsumerStreamAny(#[from] AnyError),
    #[error("ConsumerStreamNoNewEntity")]
    ConsumerStreamNoNewEntity,
    #[error("ConsumerStreamNoOldEntity")]
    ConsumerStreamNoOldEntity,
    #[error("ConsumerStreamNoNewMetadata")]
    ConsumerStreamNoNewMetadata,
    #[error("ConsumerStreamNoOldMetadata")]
    ConsumerStreamNoOldMetadata,
    #[error("ConsumerNoSchema")]
    ConsumerStreamNoSchema,
    #[error("ConsumerNoSource")]
    ConsumerStreamNoSource,
}
