use crate::decoders::{EntityRecordBody, EntityRecordMetadata};
use crate::processors::ReconciliationMsg;
use crate::resources::{EntityType, ProcessorTopic};
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::consumer::{ConsumerConfig, ConsumerConfigBuilder, ConsumerError, StreamConsumer};
use data_stream::stream::StreamPayload;
use data_stream::RDKafkaLogLevel;
use derive_new::new;
use std::sync::Arc;

const NAME: &'static str = "EntityRecords";

#[derive(Clone, new)]
pub struct EntityRecordsConsumer {
    entity: Arc<EntityType>,
    topics: Vec<Arc<ProcessorTopic>>,
}

impl StreamConsumer<EntityRecordBody, EntityRecordMetadata, ReconciliationMsg>
    for EntityRecordsConsumer
{
    fn config(&self, builder: &mut ConsumerConfigBuilder) -> Result<ConsumerConfig, AnyError> {
        let kafka = &Settings::get().kafka;
        builder
            .name(NAME)
            .brokers(kafka.config.brokers.clone())
            .group_id(&kafka.config.group_id)
            .log_level(RDKafkaLogLevel::Debug)
            .offset_reset("earliest")
            .timeout(kafka.config.timeout)
            .topics(
                self.topics
                    .iter()
                    .map(|topic| topic.to_string())
                    .collect::<Vec<String>>(),
            );

        if let Some(debug) = kafka.config.debug.as_ref() {
            builder.debug(debug);
        }

        builder.build().map_err(|e| anyhow!(e))
    }

    fn into_input_stream(
        &self,
        stream: StreamPayload<EntityRecordBody, EntityRecordMetadata>,
    ) -> Result<ReconciliationMsg, AnyError> {
        let body = stream
            .body(1)
            .ok_or_else(|| ConsumerError::ConsumerStreamNoBody)?;
        let metadata = stream
            .metadata()
            .ok_or_else(|| ConsumerError::ConsumerStreamNoMetadata)?;

        Ok(ReconciliationMsg::Match(
            self.entity.clone(),
            body.clone(),
            metadata.clone(),
        ))
    }
}
