use data_stream::producer::{ProducerConfig, ProducerConfigBuilder};
use data_stream::producer::{ProducerError, StreamProducer};

const NAME: &'static str = "ActivatedMappingProducer";

#[derive(Clone, new)]
pub struct ActivatedMappingProducer;

impl StreamProducer for ActivatedMappingProducer {
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
            .topic("calions-int-evt-activated_mappings")
            .build()
            .map_err(|e| ProducerError::ProducerConfig(NAME, e))
    }
}
