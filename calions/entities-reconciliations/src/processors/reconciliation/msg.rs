use crate::decoders::ReconciliationCommandBody;
use crate::decoders::{EntityRecordBody, EntityRecordMetadata};
use crate::processors::ReconciliationProcessorSender;
use crate::resources::EntityType;
use data_stream::stream::InputStream;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum ReconciliationMsg {
    Configure(ReconciliationCommandBody),
    Match(Arc<EntityType>, EntityRecordBody, EntityRecordMetadata),
    SelfSender(ReconciliationProcessorSender),
}

impl InputStream for ReconciliationMsg {}
