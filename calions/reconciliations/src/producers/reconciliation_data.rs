use crate::encoders::{ReconciliationDataBody, ReconciliationDataMetadata};
use crate::resources::ReconciliationDataRecord;
use crate::Settings;
use anyhow::{anyhow, Error as AnyError};
use data_stream::producer::StreamProducer;
use data_stream::producer::{ProducerConfig, ProducerConfigBuilder, ProducerSender};
use data_stream::RDKafkaLogLevel;
use derive_new::new;

const NAME: &'static str = "ReconciliationDataProducer";

pub type ReconciliationDataProducerSender =
    ProducerSender<ReconciliationDataRecord, ReconciliationDataBody, ReconciliationDataMetadata>;

#[derive(Clone, new)]
pub struct ReconciliationDataProducer;

impl StreamProducer for ReconciliationDataProducer {
    fn config(&self, builder: &mut ProducerConfigBuilder) -> Result<ProducerConfig, AnyError> {
        let kafka = &Settings::get().kafka;
        builder
            .name(NAME)
            .brokers(kafka.config.brokers.clone())
            .group_id(kafka.config.group_id.clone())
            .log_level(RDKafkaLogLevel::Debug)
            .timeout(kafka.config.timeout)
            .topic(&*kafka.producers.reconciliation_data.topic);

        if let Some(debug) = kafka.producers.reconciliation_data.debug.as_ref() {
            builder.debug(debug);
        }

        if let Some(debug) = kafka.config.debug.as_ref() {
            builder.debug(debug);
        }

        builder.build().map_err(|e| anyhow!(e))
    }
}
