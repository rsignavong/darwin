use crate::decoders::ImportInstanceBody;
use data_stream::stream::InputStream;

#[derive(Clone, Debug)]
pub enum ImportationMsg {
    ImportInstance(ImportInstanceBody),
}

impl InputStream for ImportationMsg {}
