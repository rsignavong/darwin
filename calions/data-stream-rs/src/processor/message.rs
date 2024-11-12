use crate::consumer::ConsumerEvent;
use crate::stream::{InputMessage, InputStream};

#[derive(Debug)]
pub enum ProcessorMessage<IS>
where
    IS: InputStream,
{
    Drop,
    Event(ConsumerEvent),
    Input(InputMessage<IS>),
}
