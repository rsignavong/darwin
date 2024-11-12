use crate::decoders::ActivatedMappingBody;
use crate::decoders::AnonymizationBody;
use crate::decoders::{IngestionDataBody, IngestionDataMetadata};
use crate::decoders::{ReconciliationRecordBody, ReconciliationRecordMetadata};
use crate::processors::IngestionProcessorSender;
use data_stream::stream::InputStream;

#[derive(Clone, Debug)]
pub enum IngestionMsg {
    Anonymize(AnonymizationBody),
    Ingest(ReconciliationRecordBody, ReconciliationRecordMetadata),
    Mapping(ActivatedMappingBody),
    ReloadState(IngestionDataBody, IngestionDataMetadata),
    SelfSender(IngestionProcessorSender),
}

impl InputStream for IngestionMsg {}
