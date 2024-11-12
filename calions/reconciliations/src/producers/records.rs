use crate::encoders::{RecordBody, RecordMetadata};
use crate::resources::{ProcessorTopic, Record};
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::producer::StreamProducer;
use data_stream::producer::{ProducerConfig, ProducerConfigBuilder, ProducerSender};
use data_stream::RDKafkaLogLevel;
use derive_new::new;
use std::sync::Arc;

const NAME: &'static str = "RecordsProducer";

pub type RecordsProducerSender = ProducerSender<Record, RecordBody, RecordMetadata>;

#[derive(Clone, new)]
pub struct RecordsProducer {
    topic: Arc<ProcessorTopic>,
}

impl StreamProducer for RecordsProducer {
    fn config(&self, builder: &mut ProducerConfigBuilder) -> Result<ProducerConfig, AnyError> {
        let kafka = &Settings::get().kafka;
        builder
            .name(NAME)
            .brokers(kafka.config.brokers.clone())
            .group_id(kafka.config.group_id.clone())
            .log_level(RDKafkaLogLevel::Debug)
            .timeout(kafka.config.timeout)
            .topic(&**self.topic);

        if let Some(debug) = kafka.config.debug.as_ref() {
            builder.debug(debug);
        }

        builder.build().map_err(|e| anyhow!(e))
    }
}
