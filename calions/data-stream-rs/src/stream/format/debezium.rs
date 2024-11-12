use crate::consumer::ConsumerError;
use crate::stream::{deserialize_object, serialize_object};
use crate::stream::{ProcessingTime, StreamEvent, StreamFormat};
use derive_more::Display;
use derive_new::new;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Deserialize, Display, Serialize, new)]
#[display(fmt = "Entity: {:?}\nMetadata: {:#?}", entity, metadata)]
pub struct DebeziumEntity<E, M> {
    #[serde(flatten)]
    entity: E,
    #[serde(bound(deserialize = "M: Deserialize<'de>", serialize = "M: Serialize"))]
    #[serde(
        deserialize_with = "deserialize_object",
        serialize_with = "serialize_object"
    )]
    metadata: Option<M>,
}

#[derive(Debug, Deserialize, Display, Serialize, new)]
#[display(fmt = "Op: {:?}\nBefore: {:#?}\nAfter: {:#?}", op, before, after)]
pub struct DebeziumPayload<SO, E, M> {
    before: Option<DebeziumEntity<E, M>>,
    after: Option<DebeziumEntity<E, M>>,
    source: SO,
    #[serde(
        deserialize_with = "deserialize_event",
        serialize_with = "serialize_event"
    )]
    op: StreamEvent,
    ts_ms: Option<ProcessingTime>,
}

#[derive(Debug, Deserialize, Display, Serialize, new)]
#[display(fmt = "Payload\n{}", payload)]
pub struct Debezium<SM, SO, E, M> {
    schema: Option<SM>,
    payload: DebeziumPayload<SO, E, M>,
}

impl<SM, SO, E, M> StreamFormat<SM, SO, E, M> for Debezium<SM, SO, E, M> {
    fn event(&self) -> &StreamEvent {
        &self.payload.op
    }

    fn old(&self) -> Option<(&E, Option<&M>)> {
        self.payload
            .before
            .as_ref()
            .map(|e| (&e.entity, e.metadata.as_ref()))
    }

    fn new(&self) -> Option<(&E, Option<&M>)> {
        self.payload
            .after
            .as_ref()
            .map(|e| (&e.entity, e.metadata.as_ref()))
    }

    fn schema(&self) -> Option<&SM> {
        self.schema.as_ref()
    }

    fn source(&self) -> &SO {
        &self.payload.source
    }

    fn time(&self) -> Option<&ProcessingTime> {
        self.payload.ts_ms.as_ref()
    }
}

fn deserialize_event<'de, D>(d: D) -> Result<StreamEvent, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(d)?;
    match s {
        "c" => Ok(StreamEvent::Created),
        "d" => Ok(StreamEvent::Deleted),
        "u" => Ok(StreamEvent::Updated),
        "r" => Ok(StreamEvent::Read),
        op => Err(ConsumerError::ConsumerDebeziumDeserializeEvent(op.into())),
    }
    .map_err(serde::de::Error::custom)
}

fn serialize_event<S>(event: &StreamEvent, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let op = match event {
        StreamEvent::Created => "c",
        StreamEvent::Deleted => "d",
        StreamEvent::Updated => "u",
        StreamEvent::Read => "r",
    };
    s.serialize_str(op)
}
