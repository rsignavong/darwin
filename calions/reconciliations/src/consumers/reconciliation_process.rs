use crate::decoders::{ReconciliationProcessBody, ReconciliationProcessMetadata};
use crate::processors::ReconciliationMsg;
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::consumer::{ConsumerConfig, ConsumerConfigBuilder, ConsumerError, StreamConsumer};
use data_stream::stream::StreamPayload;
use data_stream::RDKafkaLogLevel;
use derive_new::new;

const NAME: &'static str = "ReconciliationProcessConsumer";

#[derive(Clone, new)]
pub struct ReconciliationProcessConsumer;

impl StreamConsumer<ReconciliationProcessBody, ReconciliationProcessMetadata, ReconciliationMsg>
    for ReconciliationProcessConsumer
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
                kafka
                    .consumers
                    .reconciliation_process
                    .topics
                    .iter()
                    .map(|topic| topic.to_string())
                    .collect::<Vec<String>>(),
            );

        if let Some(debug) = kafka.consumers.reconciliation_process.debug.as_ref() {
            builder.debug(debug);
        }

        if let Some(debug) = kafka.config.debug.as_ref() {
            builder.debug(debug);
        }

        builder.build().map_err(|e| anyhow!(e))
    }

    fn into_input_stream(
        &self,
        stream: StreamPayload<ReconciliationProcessBody, ReconciliationProcessMetadata>,
    ) -> Result<ReconciliationMsg, AnyError> {
        let reconciliation_process = stream
            .body(1)
            .ok_or_else(|| ConsumerError::ConsumerStreamNoBody)?;

        Ok(ReconciliationMsg::Activate(reconciliation_process.clone()))
    }
}
