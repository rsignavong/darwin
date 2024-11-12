use super::RawRecordMsg;
use crate::encoders::{RawRecordBody, RawRecordMetadata};
use crate::resources::RawRecord;
use crate::wizard::Context;
use crossbeam::channel::Sender;
use crossbeam::sync::Unparker;
use data_stream::processor::{ProcessorConfig, ProcessorConfigBuilder};
use data_stream::processor::{ProcessorError, StreamProcessor};
use data_stream::stream::OutputMessage;
use failure::Error as FailureError;

const NAME: &'static str = "RawRecordProcessor";

#[derive(new)]
pub struct RawRecordProcessor {
    raw_record: RawRecord,
    producer: Sender<OutputMessage<RawRecord, RawRecordBody, RawRecordMetadata>>,
}

impl StreamProcessor<RawRecordMsg> for RawRecordProcessor {
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
        _message: &RawRecordMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn deleted(
        &mut self,
        _message: &RawRecordMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn updated(
        &mut self,
        _message: &RawRecordMsg,
        _unparker: &Unparker,
    ) -> Result<(), ProcessorError> {
        Ok(())
    }

    fn custom(
        &mut self,
        RawRecordMsg(activated_mapping_id, context): &RawRecordMsg,
        _unparker: Option<&Unparker>,
    ) -> Result<(), ProcessorError> {
        match context {
            Context::Web { cookie, phone } => self.raw_record.generate_web(
                *activated_mapping_id,
                cookie.to_owned(),
                phone.to_owned(),
            ),
            Context::CRM { email, phone } => self.raw_record.generate_crm(
                *activated_mapping_id,
                email.to_owned(),
                phone.to_owned(),
            ),
            Context::Newsletter { email, phone } => self.raw_record.generate_news(
                *activated_mapping_id,
                email.to_owned(),
                phone.to_owned(),
            ),
        }
        self.producer
            .send(OutputMessage::new(self.raw_record.clone(), None))
            .map_err(|e| {
                ProcessorError::ProcessorCustom(NAME, FailureError::from_boxed_compat(Box::new(e)))
            })
    }
}
