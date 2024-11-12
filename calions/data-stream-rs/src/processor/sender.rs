use super::ProcessorMessage;
use async_channel::Sender;

pub type ProcessorSender<IS> = Sender<ProcessorMessage<IS>>;
