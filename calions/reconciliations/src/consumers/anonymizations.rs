use crate::decoders::{AnonymizationBody, AnonymizationMetadata};
use crate::processors::ReconciliationMsg;
use crate::resources::ProcessorTopic;
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::consumer::{ConsumerConfig, ConsumerConfigBuilder, ConsumerError, StreamConsumer};
use data_stream::stream::StreamPayload;
use data_stream::RDKafkaLogLevel;
use derive_new::new;
use std::sync::Arc;

const NAME: &'static str = "AnonymizationsConsumer";

#[derive(Clone, new)]
pub struct AnonymizationsConsumer {
    topics: Arc<Vec<ProcessorTopic>>,
}

impl StreamConsumer<AnonymizationBody, AnonymizationMetadata, ReconciliationMsg>
    for AnonymizationsConsumer
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
        stream: StreamPayload<AnonymizationBody, AnonymizationMetadata>,
    ) -> Result<ReconciliationMsg, AnyError> {
        let body = stream
            .body(1)
            .ok_or_else(|| ConsumerError::ConsumerStreamNoBody)?;

        Ok(ReconciliationMsg::Anonymize(body.clone()))
    }
}
