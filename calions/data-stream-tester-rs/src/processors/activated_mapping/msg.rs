use crate::wizard::Mapping;
use data_stream::stream::InputStream;

pub struct ActivatedMappingMsg(pub Mapping);

impl InputStream for ActivatedMappingMsg {}
