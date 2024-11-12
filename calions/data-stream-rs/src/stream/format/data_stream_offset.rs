use crate::common::TopicPartition;
use crate::stream::{Offset, ProcessingTime, StreamEvent, StreamFormat};
use derive_more::Display;
use derive_new::new;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

#[derive(Debug, Deserialize, Display, Serialize, new)]
#[display(fmt = "Offset\n{}:{}", topic_partition, offset)]
pub struct DataStreamOffset<SM, SO, E, M> {
    name: SO,
    topic_partition: TopicPartition,
    offset: Offset,
    time: ProcessingTime,
    phantom_sm: PhantomData<SM>,
    phantom_e: PhantomData<E>,
    phantom_m: PhantomData<M>,
}

impl<SM, SO, E, M> StreamFormat<SM, SO, E, M> for DataStreamOffset<SM, SO, E, M> {
    fn event(&self) -> &StreamEvent {
        &StreamEvent::Created
    }

    fn old(&self) -> Option<(&E, Option<&M>)> {
        None
    }

    fn new(&self) -> Option<(&E, Option<&M>)> {
        None
    }

    fn schema(&self) -> Option<&SM> {
        None
    }

    fn source(&self) -> &SO {
        &self.name
    }

    fn time(&self) -> Option<&ProcessingTime> {
        Some(&self.time)
    }
}

impl<SM, SO, E, M> DataStreamOffset<SM, SO, E, M> {
    pub fn offset(&self) -> &Offset {
        &self.offset
    }

    pub fn topic_partition(&self) -> &TopicPartition {
        &self.topic_partition
    }
}
