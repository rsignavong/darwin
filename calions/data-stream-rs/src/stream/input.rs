use super::{StreamEvent, StreamNext};
use derivative::Derivative;
use derive_new::new;

pub trait InputStream {}

#[derive(Clone, Copy, Debug)]
pub enum InputEvent {
    Custom,
    Stream(StreamEvent),
}

#[derive(new, Derivative)]
#[derivative(Debug)]
pub struct InputMessage<IS> {
    event: InputEvent,
    #[derivative(Debug = "ignore")]
    message: IS,
    #[derivative(Debug = "ignore")]
    next: Option<StreamNext>,
}

impl<IS> InputMessage<IS>
where
    IS: InputStream,
{
    pub fn event(&self) -> InputEvent {
        self.event
    }

    pub fn message(&self) -> &IS {
        &self.message
    }

    pub fn next(&self) -> Option<&StreamNext> {
        self.next.as_ref()
    }
}
