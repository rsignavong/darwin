use super::ReconciliationSagaMsg;
use crate::encoders::{ReconciliationSagaBody, ReconciliationSagaMetadata};
use crate::resources::ReconciliationSaga;
use crossbeam::channel::Sender;
use crossbeam::sync::Unparker;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorError, StreamProcessor};
use data_stream::stream::OutputMessage;
use failure::Error as FailureError;

const NAME: &'static str = "ReconciliationSagaProcessor";

#[derive(new)]
pub struct ReconciliationSagaProcessor {
    reconciliation_saga: ReconciliationSaga,
    producer: Sender<
        OutputMessage<ReconciliationSaga, ReconciliationSagaBody, ReconciliationSagaMetadata>,
    >,
}

impl StreamProcessor<ReconciliationSagaMsg> for ReconciliationSagaProcessor {
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
        _message: &ReconciliationSagaMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn deleted(
        &mut self,
        _message: &ReconciliationSagaMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn updated(
        &mut self,
        _message: &ReconciliationSagaMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn custom(
        &mut self,
        ReconciliationSagaMsg(processor_id, activated_mapping_id): &ReconciliationSagaMsg,
        _unparker: Option<&Unparker>,
    ) -> Result<(), ProcessorError> {
        self.reconciliation_saga
            .generate(*processor_id, *activated_mapping_id);
        self.producer
            .send(OutputMessage::new(self.reconciliation_saga.clone(), None))
            .map_err(|e| {
                ProcessorError::ProcessorCustom(NAME, FailureError::from_boxed_compat(Box::new(e)))
            })
    }
}
