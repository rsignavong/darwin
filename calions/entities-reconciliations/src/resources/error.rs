use crate::resources::NodeId;
use data_stream::consumer::ConsumerError;
use std::sync::Arc;
use thiserror::Error;
use ulid::MonotonicError as UlidError;

#[derive(Debug, Error)]
pub enum ResourcesError {
    #[error("EntityRecordId")]
    EntityRecordId,
    #[error("ReconciliationDeleteMissingActivatedMappingId")]
    ReconciliationDeleteMissingActivatedMappingId,
    #[error("ReconciliationDeleteMissingAggregates")]
    ReconciliationDeleteMissingAggregates,
    #[error("ReconciliationDeleteMissingGraph")]
    ReconciliationDeleteMissingGraph,
    #[error("ReconciliationDeleteMissingOrder")]
    ReconciliationDeleteMissingOrder,
    #[error("AggregateId {0}")]
    AggregateId(UlidError),
    #[error("NodeId {0}")]
    NodeId(UlidError),
    #[error("NodeConnectedSet")]
    NodeConnectedSet,
    #[error("NodesConnectedListMapEntityNotFound")]
    NodesConnectedListMapEntityNotFound(Arc<NodeId>),
    #[error("ReconciliationMatchMissingActivatedMappingId")]
    ReconciliationMatchMissingActivatedMappingId,
    #[error("ReconciliationMatchMissingAdjacencies")]
    ReconciliationMatchMissingAdjacencies,
    #[error("ReconciliationMatchMissingAggregateMap")]
    ReconciliationMatchMissingAggregateMap,
    #[error("ReconciliationMatchMissingFields")]
    ReconciliationMatchMissingFields,
    #[error("ReconciliationMatchMissingGraph")]
    ReconciliationMatchMissingGraph,
    #[error("ReconciliationMatchMissingNodes")]
    ReconciliationMatchMissingNodes,
    #[error("ReconciliationMatchRecordOnCreatedUpdated")]
    ReconciliationMatchRecordOnCreatedUpdated,
    #[error("ReconciliationMatchRecordOnDeleted")]
    ReconciliationMatchRecordOnDeleted,
    #[error("ReconciliationMatchMissingOrder")]
    ReconciliationMatchMissingOrder,
    #[error("ReconciliationMissingSelfProcessorSender")]
    ReconciliationMissingSelfProcessorSender,
    #[error("ReconciliationRawRecordsConsumerSpawn")]
    ReconciliationRawRecordsConsumerSpawn { source: ConsumerError },
    #[error("RelationshipId {0}")]
    RelationshipId(UlidError),
}
