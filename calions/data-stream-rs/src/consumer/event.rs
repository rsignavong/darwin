#[derive(Clone, Debug)]
pub enum ConsumerEvent {
    Starting(String),
    Terminating(String),
}
