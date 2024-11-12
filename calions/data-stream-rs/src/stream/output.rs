use super::{Debezium, DebeziumEntity, DebeziumPayload};
use super::{OutputStreamError, OutputStreamFormatError, OutputStreamKeyError};
use super::{OutputStreamEventError, OutputStreamNewError, OutputStreamOldError};
use super::{OutputStreamSchemaError, OutputStreamSourceError, OutputStreamTimeError};
use super::{ProcessingTime, StreamEvent, StreamPayload};
use crate::stream::StreamNext;
use derivative::Derivative;
use derive_new::new;
use serde::Serialize;
use std::fmt::Debug;
use std::marker::PhantomData;

pub trait OutputStream<SM, SO, E, M>
where
    SM: Debug + Serialize,
    SO: Debug + Serialize,
    E: Debug + Serialize,
    M: Debug + Serialize,
{
    fn event(&self) -> Result<StreamEvent, OutputStreamEventError> {
        let event = match (self.old()?, self.new()?) {
            (None, None) => StreamEvent::Read,
            (None, Some(_)) => StreamEvent::Created,
            (Some(_), None) => StreamEvent::Deleted,
            (Some(_), Some(_)) => StreamEvent::Updated,
        };

        Ok(event)
    }

    fn format(&self) -> Result<Option<OutputFormat>, OutputStreamFormatError> {
        Ok(None)
    }

    fn key(&self) -> Result<String, OutputStreamKeyError>;
    fn new(&self) -> Result<Option<(E, Option<M>)>, OutputStreamNewError>;
    fn old(&self) -> Result<Option<(E, Option<M>)>, OutputStreamOldError>;

    fn schema(&self) -> Result<Option<SM>, OutputStreamSchemaError> {
        Ok(None)
    }

    fn source(&self) -> Result<SO, OutputStreamSourceError>;

    fn time(&self) -> Result<Option<ProcessingTime>, OutputStreamTimeError> {
        Ok(Some(ProcessingTime::new()))
    }
}

#[derive(Clone, Debug, Derivative)]
#[derivative(Default)]
pub enum OutputFormat {
    #[derivative(Default)]
    Debezium,
}

#[derive(Debug, new)]
pub struct OutputMessage<O, SM, SO, E, M>
where
    O: OutputStream<SM, SO, E, M>,
    SM: Debug + Serialize,
    SO: Debug + Serialize,
    E: Debug + Serialize,
    M: Debug + Serialize,
{
    stream: O,
    next: Option<StreamNext>,
    phantom_sm: PhantomData<SM>,
    phantom_so: PhantomData<SO>,
    phantom_e: PhantomData<E>,
    phantom_m: PhantomData<M>,
}

impl<O, SM, SO, E, M> OutputMessage<O, SM, SO, E, M>
where
    O: OutputStream<SM, SO, E, M>,
    SM: Debug + Serialize,
    SO: Debug + Serialize,
    E: Debug + Serialize,
    M: Debug + Serialize,
{
    pub(crate) fn stream(&self) -> &O {
        &self.stream
    }

    pub(crate) fn next(&self) -> Option<&StreamNext> {
        self.next.as_ref()
    }

    pub(crate) fn to_json(
        &self,
        default: &OutputFormat,
    ) -> Result<Option<String>, OutputStreamError> {
        let format = self.stream.format()?.unwrap_or_else(|| default.clone());

        let s = match format {
            OutputFormat::Debezium => {
                let after = self.stream.new()?.map(|(e, m)| DebeziumEntity::new(e, m));
                let before = self.stream.old()?.map(|(e, m)| DebeziumEntity::new(e, m));
                let op = self.stream.event()?;
                let source = self.stream.source()?;
                let time = self.stream.time()?;
                let payload = DebeziumPayload::new(before, after, source, op, time);
                let stream_payload =
                    &StreamPayload::Debezium(Debezium::new(self.stream.schema()?, payload));
                Some(serde_json::to_string(stream_payload)?)
            }
        };

        Ok(s)
    }
}
