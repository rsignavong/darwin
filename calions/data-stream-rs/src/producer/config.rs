use crate::stream::OutputFormat;
use derive_builder::Builder;
use rdkafka::config::RDKafkaLogLevel;

#[derive(Clone, Builder, Debug)]
pub struct ProducerConfig {
    pub(crate) name: &'static str,
    #[builder(setter(into))]
    pub(crate) brokers: Vec<String>,
    #[builder(setter(into, strip_option), default)]
    pub(crate) debug: Option<String>,
    #[builder(default)]
    pub(crate) format: OutputFormat,
    #[builder(setter(into))]
    pub(crate) group_id: String,
    #[builder(setter(strip_option), default)]
    pub(crate) log_level: Option<RDKafkaLogLevel>,
    pub(crate) timeout: u64,
    #[builder(setter(into))]
    pub(crate) topic: String,
}
