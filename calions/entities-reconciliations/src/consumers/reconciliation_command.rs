use crate::decoders::{ReconciliationCommandBody, ReconciliationCommandMetadata};
use crate::processors::ReconciliationMsg;
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::consumer::{ConsumerConfig, ConsumerConfigBuilder, ConsumerError, StreamConsumer};
use data_stream::stream::StreamPayload;
use data_stream::RDKafkaLogLevel;
use derive_new::new;

const NAME: &'static str = "ReconciliationCommandConsumer";

#[derive(Clone, new)]
pub struct ReconciliationCommandConsumer;

impl StreamConsumer<ReconciliationCommandBody, ReconciliationCommandMetadata, ReconciliationMsg>
    for ReconciliationCommandConsumer
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
                    .reconciliation_command
                    .topics
                    .iter()
                    .map(|topic| topic.to_string())
                    .collect::<Vec<String>>(),
            );

        if let Some(debug) = kafka.consumers.reconciliation_command.debug.as_ref() {
            builder.debug(debug);
        }

        if let Some(debug) = kafka.config.debug.as_ref() {
            builder.debug(debug);
        }

        builder.build().map_err(|e| anyhow!(e))
    }

    fn into_input_stream(
        &self,
        stream: StreamPayload<ReconciliationCommandBody, ReconciliationCommandMetadata>,
    ) -> Result<ReconciliationMsg, AnyError> {
        let reconciliation_command = stream
            .body(1)
            .ok_or_else(|| ConsumerError::ConsumerStreamNoBody)?;

        Ok(ReconciliationMsg::Configure(reconciliation_command.clone()))
    }
}
