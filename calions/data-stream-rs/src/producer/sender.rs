use super::ProducerMessage;
use async_channel::{Receiver, Sender};

pub(crate) type ProducerReceiver<O, SM, SO, E, M> = Receiver<ProducerMessage<O, SM, SO, E, M>>;

pub type ProducerSender<O, SM, SO, E, M> = Sender<ProducerMessage<O, SM, SO, E, M>>;
