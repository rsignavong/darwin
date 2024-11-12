use data_stream::producer::{ProducerConfig, ProducerConfigBuilder};
use data_stream::producer::{ProducerError, StreamProducer};

const NAME: &'static str = "ReconciliationDataProducer";

#[derive(Clone, new)]
pub struct ReconciliationDataProducer;

impl StreamProducer for ReconciliationDataProducer {
    fn config(&self, builder: &mut ProducerConfigBuilder) -> Result<ProducerConfig, ProducerError> {
        builder
            .name(NAME)
            .brokers(
                vec!["localhost:9092"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<String>>(),
            )
            .timeout(5_000)
            .topic("calions-int-evt-reconciliation_data")
            .build()
            .map_err(|e| ProducerError::ProducerConfig(NAME, e))
    }
}

