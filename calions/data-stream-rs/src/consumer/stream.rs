use super::error::ConsumerError;
use crate::common::TopicPartition;
use crate::stream::{Offset as StreamOffset, StreamPayload};
use rdkafka::error::KafkaError;
use rdkafka::message::{BorrowedMessage, Message};
use rdkafka::topic_partition_list::{Offset, TopicPartitionList};
use serde::de::DeserializeOwned;
use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub(crate) struct ConsumerStream<SM, SO, E, M>
where
    SM: Debug,
    SO: Debug,
    E: Debug,
    M: Debug,
{
    pub offset: StreamOffset,
    pub payload: StreamPayload<SM, SO, E, M>,
    pub topic_partition: TopicPartition,
    pub topic_partition_list: TopicPartitionList,
}

impl<SM, SO, E, M> Display for ConsumerStream<SM, SO, E, M>
where
    SM: Debug,
    SO: Debug,
    E: Debug,
    M: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let elements = self.topic_partition_list.elements();
        let (topic, partition) = if let Some(elm) = elements.first() {
            (elm.topic(), elm.partition())
        } else {
            ("Unknown", -1)
        };

        write!(
            f,
            "Topic: {}\nPartition: {}\n{}",
            topic, partition, self.payload
        )
    }
}

impl<SM, SO, E, M> TryFrom<Result<BorrowedMessage<'_>, KafkaError>> for ConsumerStream<SM, SO, E, M>
where
    SM: Debug + DeserializeOwned,
    SO: Debug + DeserializeOwned,
    E: Debug + DeserializeOwned,
    M: Debug + DeserializeOwned,
{
    type Error = ConsumerError;

    fn try_from(message: Result<BorrowedMessage<'_>, KafkaError>) -> Result<Self, Self::Error> {
        let borrowed_message =
            message.map_err(|source| ConsumerError::ConsumerBorrowedMessage { source })?;
        let mut topic_partition_list = TopicPartitionList::new();
        topic_partition_list.add_partition_offset(
            borrowed_message.topic(),
            borrowed_message.partition(),
            Offset::Offset(borrowed_message.offset()),
        );
        let topic_partition = TopicPartition::new(
            borrowed_message.topic().to_owned(),
            borrowed_message.partition(),
        );
        let offset: StreamOffset = borrowed_message.offset().into();
        let payload_view = borrowed_message
            .payload_view::<str>()
            .ok_or_else(|| ConsumerError::ConsumerStreamNoBytes)?
            .map_err(|source| ConsumerError::ConsumerStreamDecode { source })?;
        let payload = serde_json::from_str::<StreamPayload<SM, SO, E, M>>(payload_view)
            .map_err(|source| ConsumerError::ConsumerStreamPayload { source })?;

        Ok(ConsumerStream {
            offset,
            payload,
            topic_partition,
            topic_partition_list,
        })
    }
}
