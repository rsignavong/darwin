use data_stream::consumer::ConsumerError;
use std::convert::Infallible;
use thiserror::Error;
use ulid::{DecodeError as UlidDecodeError, MonotonicError as UlidError};

#[derive(Debug, Error)]
pub enum ResourcesError {
    #[error("ActivatedMappingMappingField")]
    ActivatedMappingMappingField { source: Infallible },
    #[error("MappingMatchingId {0}")]
    MappingMatchingId(UlidError),
    #[error("MappingMatchingTryFromStr {0}")]
    MappingMatchingTryFromStr(UlidDecodeError),
    #[error("RawRecordBodyMappingField")]
    RawRecordBodyMappingField { source: Infallible },
    #[error("RawRecordBodyRecordValue")]
    RawRecordBodyRecordValue { source: Infallible },
    #[error("ProfileIdGeneration {0}")]
    ProfileIdGeneration(UlidError),
    #[error("ReconciliationAnonymizationsConsumerSpawn")]
    ReconciliationAnonymizationsConsumerSpawn { source: ConsumerError },
    #[error("ReconciliationKeyDeserializeRecordValue")]
    ReconciliationKeyDeserializeRecordValue,
    #[error("ReconciliationKeyDeserializeMappingMatchingId")]
    ReconciliationKeyDeserializeMappingMatchingId,
    #[error("ReconciliationKeyInvalid")]
    ReconciliationKeyInvalid,
    #[error("ReconciliationMatchMissingActivatedMappingId")]
    ReconciliationMatchMissingActivatedMappingId,
    #[error("ReconciliationMatchMissingRecordsProducerTopic")]
    ReconciliationMatchMissingRecordsProducerTopic,
    #[error("ReconciliationMatchMissingActivatedMapping")]
    ReconciliationMatchMissingActivatedMapping,
    #[error("ReconciliationMatchMissingOrganizationId")]
    ReconciliationMatchMissingOrganizationId,
    #[error("ReconciliationMissingActivatedMappingId")]
    ReconciliationMissingActivatedMappingId,
    #[error("ReconciliationMissingSelfProcessorSender")]
    ReconciliationMissingSelfProcessorSender,
    #[error("ReconciliationProcessConsumerSpawn")]
    ReconciliationProcessConsumerSpawn { source: ConsumerError },
    #[error("ReconciliationRawRecordsConsumerSpawn")]
    ReconciliationRawRecordsConsumerSpawn { source: ConsumerError },
    #[error("ReconciliationSetMappingMissingActivatedMappingId")]
    ReconciliationSetMappingMissingActivatedMappingId,
    #[error("ReconciliationSetMappingMissingOrganizationId")]
    ReconciliationSetMappingMissingOrganizationId,
    #[error("ReconciliationTopic")]
    ReconciliationTopic { source: Infallible },
    #[error("TransactionIdGeneration {0}")]
    TransactionIdGeneration(UlidError),
}
