use ulid::Ulid;

mod activated_mapping;
mod raw_record;
mod reconciliation_data;
mod reconciliation_saga;

pub type ActivatedMappingId = Ulid;
pub type ContextId = Ulid;
pub type ProcessorId = Ulid;
pub type RawRecordId = Ulid;
pub type ReconciliationDataId = Ulid;
pub type ReconciliationSagaId = Ulid;

pub type ContextName = String;
pub type ContextMatchingId = String;
pub type MappingField = String;
pub type RecordValue = String;

pub use activated_mapping::ActivatedMapping;
pub use raw_record::RawRecord;
pub use reconciliation_data::ReconciliationData;
pub use reconciliation_saga::ReconciliationSaga;
