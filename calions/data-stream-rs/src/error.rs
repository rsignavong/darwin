use super::consumer::ConsumerError;
use super::processor::ProcessorError;
use super::producer::ProducerError;
use anyhow::Error as AnyError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataStreamError {
    #[error("Consumer")]
    Consumer(#[from] ConsumerError),
    #[error("Processor")]
    Processor(#[from] ProcessorError),
    #[error("Producer")]
    Producer(#[from] ProducerError),
    #[error(transparent)]
    Other(#[from] AnyError),
}
