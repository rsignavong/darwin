use super::ActivatedMappingMsg;
use crate::encoders::{ActivatedMappingBody, ActivatedMappingMetadata};
use crate::resources::ActivatedMapping;
use crossbeam::channel::Sender;
use crossbeam::sync::Unparker;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorError, StreamProcessor};
use data_stream::stream::OutputMessage;
use failure::Error as FailureError;

const NAME: &'static str = "ActivatedMappingProcessor";

#[derive(new)]
pub struct ActivatedMappingProcessor {
    activated_mapping: ActivatedMapping,
    producer:
        Sender<OutputMessage<ActivatedMapping, ActivatedMappingBody, ActivatedMappingMetadata>>,
}

impl StreamProcessor<ActivatedMappingMsg> for ActivatedMappingProcessor {
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
        _message: &ActivatedMappingMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn deleted(
        &mut self,
        _message: &ActivatedMappingMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn updated(
        &mut self,
        _message: &ActivatedMappingMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn custom(
        &mut self,
        ActivatedMappingMsg(mapping): &ActivatedMappingMsg,
        _unparker: Option<&Unparker>,
    ) -> Result<(), ProcessorError> {
        self.activated_mapping.generate(mapping.clone());
        self.producer
            .send(OutputMessage::new(self.activated_mapping.clone(), None))
            .map_err(|e| {
                ProcessorError::ProcessorCustom(NAME, FailureError::from_boxed_compat(Box::new(e)))
            })
    }
}
