use crate::resources::ProcessorId;
use data_stream::stream::InputStream;

pub struct ReconciliationDataMsg(pub ProcessorId);

impl InputStream for ReconciliationDataMsg {}
