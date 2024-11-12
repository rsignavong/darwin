use crate::decoders::{IngestionDataBody, IngestionDataMetadata};
use crate::processors::IngestionMsg;
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::consumer::{ConsumerConfig, ConsumerConfigBuilder};
use data_stream::consumer::{ConsumerError, StreamConsumer};
use data_stream::consumer::{ConsumerHalt, ConsumerIndexStorage};
use data_stream::stream::StreamPayload;
use data_stream::RDKafkaLogLevel;
use derive_new::new;

const NAME: &'static str = "IngestionDataConsumer";

#[derive(Clone, new)]
pub struct IngestionDataConsumer;

impl StreamConsumer<IngestionDataBody, IngestionDataMetadata, IngestionMsg>
    for IngestionDataConsumer
{
    fn config(&self, builder: &mut ConsumerConfigBuilder) -> Result<ConsumerConfig, AnyError> {
        let kafka = &Settings::get().kafka;
        builder
            .name(NAME)
            .brokers(kafka.config.brokers.clone())
            .group_id(&kafka.config.group_id)
            .halt(ConsumerHalt::Auto)
            .index_storage(ConsumerIndexStorage::InMemory)
            .log_level(RDKafkaLogLevel::Debug)
            .offset_reset("earliest")
            .timeout(kafka.config.timeout)
            .topics(
                kafka
                    .consumers
                    .ingestion_data
                    .topics
                    .iter()
                    .map(|topic| topic.to_string())
                    .collect::<Vec<String>>(),
            );

        if let Some(debug) = kafka.consumers.ingestion_data.debug.as_ref() {
            builder.debug(debug);
        }

        if let Some(debug) = kafka.config.debug.as_ref() {
            builder.debug(debug);
        }

        builder.build().map_err(|e| anyhow!(e))
    }

    fn into_input_stream(
        &self,
        stream: StreamPayload<IngestionDataBody, IngestionDataMetadata>,
    ) -> Result<IngestionMsg, AnyError> {
        let ingestion_data = stream
            .body(1)
            .ok_or_else(|| ConsumerError::ConsumerStreamNoBody)?;

        let metadata = stream
            .metadata()
            .ok_or_else(|| ConsumerError::ConsumerStreamNoMetadata)?;

        Ok(IngestionMsg::ReloadState(
            ingestion_data.clone(),
            metadata.clone(),
        ))
    }
}
