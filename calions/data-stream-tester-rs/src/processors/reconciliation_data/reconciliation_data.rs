use super::ReconciliationDataMsg;
use crate::encoders::{ReconciliationDataBody, ReconciliationDataMetadata};
use crate::resources::ReconciliationData;
use crossbeam::channel::Sender;
use crossbeam::sync::Unparker;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorError, StreamProcessor};
use data_stream::stream::OutputMessage;
use failure::Error as FailureError;

const NAME: &'static str = "ReconciliationDataProcessor";

#[derive(new)]
pub struct ReconciliationDataProcessor {
    reconciliation_data: ReconciliationData,
    producer: Sender<
        OutputMessage<ReconciliationData, ReconciliationDataBody, ReconciliationDataMetadata>,
    >,
}

impl StreamProcessor<ReconciliationDataMsg> for ReconciliationDataProcessor {
    fn config(
        &self,
        builder: &mut ProcessorConfigBuilder,
    ) -> Result<ProcessorConfig, ProcessorError> {
        builder
            .name(NAME)
            .build()
            .map_err(|e| ProcessorError::ProcessorConfig(NAME, e))
    }

    fn created(
        &mut self,
        _message: &ReconciliationDataMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn deleted(
        &mut self,
        _message: &ReconciliationDataMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn updated(
        &mut self,
        _message: &ReconciliationDataMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn custom(
        &mut self,
        ReconciliationDataMsg(processor_id): &ReconciliationDataMsg,
        _unparker: Option<&Unparker>,
    ) -> Result<(), ProcessorError> {
        self.reconciliation_data.generate(*processor_id);
        self.producer
            .send(OutputMessage::new(self.reconciliation_data.clone(), None))
            .map_err(|e| {
                ProcessorError::ProcessorCustom(NAME, FailureError::from_boxed_compat(Box::new(e)))
            })
    }
}
