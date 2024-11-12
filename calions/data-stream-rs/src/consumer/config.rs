use derivative::Derivative;
use derive_builder::Builder;
use rdkafka::config::RDKafkaLogLevel;

#[derive(Clone, Builder, Debug)]
pub struct ConsumerConfig {
    pub(crate) name: &'static str,
    #[builder(setter(into))]
    pub(crate) brokers: Vec<String>,
    #[builder(setter(into, strip_option), default)]
    pub(crate) debug: Option<String>,
    #[builder(setter(into))]
    pub(crate) group_id: String,
    #[builder(default)]
    pub(crate) halt: ConsumerHalt,
    #[builder(setter(strip_option), default)]
    pub(crate) log_level: Option<RDKafkaLogLevel>,
    #[builder(setter(into))]
    pub(crate) offset_reset: String,
    pub(crate) offset_storage: ConsumerOffsetStorage,
    pub(crate) timeout: u64,
    #[builder(setter(into))]
    pub(crate) topics: Vec<String>,
}

#[derive(Clone, Debug, Derivative, Eq, PartialEq)]
#[derivative(Default)]
pub enum ConsumerHalt {
    Auto,
    #[derivative(Default)]
    Manual,
}

#[derive(Clone, Debug, Derivative, Eq, PartialEq)]
pub enum ConsumerOffsetStorage {
    KafkaProducer(String),
    InMemory,
}
