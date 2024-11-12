use data_stream::producer::{ProducerConfig, ProducerConfigBuilder};
use data_stream::producer::{ProducerError, StreamProducer};

const NAME: &'static str = "ReconciliationSagaProducer";

#[derive(Clone, new)]
pub struct ReconciliationSagaProducer;

impl StreamProducer for ReconciliationSagaProducer {
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
            .topic("calions-int-evt-reconciliation_saga")
            .build()
            .map_err(|e| ProducerError::ProducerConfig(NAME, e))
    }
}

