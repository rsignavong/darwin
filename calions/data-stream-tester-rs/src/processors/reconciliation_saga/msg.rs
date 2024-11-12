use crate::resources::{ActivatedMappingId, ProcessorId};
use data_stream::stream::InputStream;

pub struct ReconciliationSagaMsg(pub ProcessorId, pub ActivatedMappingId);

impl InputStream for ReconciliationSagaMsg {}
