use crate::stream::{OutputMessage, OutputStream};
use serde::Serialize;
use std::fmt::Debug;

pub enum ProducerMessage<O, SM, SO, E, M>
where
    O: OutputStream<SM, SO, E, M>,
    SM: Debug + Serialize,
    SO: Debug + Serialize,
    E: Debug + Serialize,
    M: Debug + Serialize,
{
    Drop,
    Output(OutputMessage<O, SM, SO, E, M>),
}
