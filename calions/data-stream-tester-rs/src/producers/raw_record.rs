use data_stream::producer::{ProducerConfig, ProducerConfigBuilder};
use data_stream::producer::{ProducerError, StreamProducer};

const NAME: &'static str = "RawRecordProducer";

#[derive(Clone, new)]
pub struct RawRecordProducer;

impl StreamProducer for RawRecordProducer {
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
            .topic("calions-int-evt-import_records")
            .build()
            .map_err(|e| ProducerError::ProducerConfig(NAME, e))
    }
}

