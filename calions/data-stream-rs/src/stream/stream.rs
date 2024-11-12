use super::format::{DataStreamOffset, Debezium};
use crate::stream::ProcessingTime;
use derive_more::Display;
use event_listener::{Event, EventListener};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StreamEvent {
    Created,
    Deleted,
    Updated,
    Read,
}

pub trait StreamFormat<SM, SO, E, M> {
    fn event(&self) -> &StreamEvent;
    fn old(&self) -> Option<(&E, Option<&M>)>;
    fn new(&self) -> Option<(&E, Option<&M>)>;
    fn schema(&self) -> Option<&SM>;
    fn source(&self) -> &SO;
    fn time(&self) -> Option<&ProcessingTime>;
}

#[derive(Clone, Debug)]
pub struct StreamNext {
    event: Arc<Event>,
}

impl StreamNext {
    pub fn new() -> (Self, EventListener) {
        let event = Arc::new(Event::new());
        let listener = event.listen();

        (StreamNext { event }, listener)
    }

    pub fn next(&self) {
        self.event.notify(1);
    }
}

#[derive(Debug, Deserialize, Display, Serialize)]
#[serde(untagged)]
pub enum StreamPayload<SM, SO, E, M>
where
    SM: Debug,
    SO: Debug,
    E: Debug,
    M: Debug,
{
    #[display(fmt = "DataStreamOffset: {}", _0)]
    DataStreamOffset(DataStreamOffset<SM, SO, E, M>),
    #[display(fmt = "Debezium: {}", _0)]
    Debezium(Debezium<SM, SO, E, M>),
}

impl<SM, SO, E, M> StreamPayload<SM, SO, E, M>
where
    SM: Debug,
    SO: Debug,
    E: Debug,
    M: Debug,
{
    pub fn event(&self) -> &StreamEvent {
        match self {
            StreamPayload::DataStreamOffset(dso) => dso.event(),
            StreamPayload::Debezium(dbz) => dbz.event(),
        }
    }

    pub fn old(&self) -> Option<(&E, Option<&M>)> {
        match self {
            StreamPayload::DataStreamOffset(dso) => dso.old(),
            StreamPayload::Debezium(dbz) => dbz.old(),
        }
    }

    pub fn new(&self) -> Option<(&E, Option<&M>)> {
        match self {
            StreamPayload::DataStreamOffset(dso) => dso.new(),
            StreamPayload::Debezium(dbz) => dbz.new(),
        }
    }

    pub fn schema(&self) -> Option<&SM> {
        match self {
            StreamPayload::DataStreamOffset(dso) => dso.schema(),
            StreamPayload::Debezium(dbz) => dbz.schema(),
        }
    }

    pub fn source(&self) -> &SO {
        match self {
            StreamPayload::DataStreamOffset(dso) => dso.source(),
            StreamPayload::Debezium(dbz) => dbz.source(),
        }
    }

    pub fn time(&self) -> Option<&ProcessingTime> {
        match self {
            StreamPayload::DataStreamOffset(dso) => dso.time(),
            StreamPayload::Debezium(dbz) => dbz.time(),
        }
    }
}
