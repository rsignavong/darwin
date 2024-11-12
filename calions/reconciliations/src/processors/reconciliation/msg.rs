use crate::decoders::AnonymizationBody;
use crate::decoders::ReconciliationProcessBody;
use crate::decoders::{RawRecordBody, RawRecordMetadata};
use crate::decoders::{ReconciliationDataBody, ReconciliationDataMetadata};
use crate::processors::ReconciliationProcessorSender;
use data_stream::stream::InputStream;

#[derive(Clone, Debug)]
pub enum ReconciliationMsg {
    Activate(ReconciliationProcessBody),
    Anonymize(AnonymizationBody),
    Match(RawRecordBody, RawRecordMetadata),
    ReloadState(ReconciliationDataBody, ReconciliationDataMetadata),
    SelfSender(ReconciliationProcessorSender),
}

impl InputStream for ReconciliationMsg {}
