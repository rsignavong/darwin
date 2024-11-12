use crate::resources::ActivatedMappingId;
use crate::wizard::Context;
use data_stream::stream::InputStream;

pub struct RawRecordMsg(pub ActivatedMappingId, pub Context);

impl InputStream for RawRecordMsg {}
