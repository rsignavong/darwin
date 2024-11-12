use super::{Manifest, Resource, ResourceName};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Error as IoError, LineWriter};

#[derive(Debug, Fail)]
pub enum ResourceWriterError {
    #[fail(display = "Failed to set writer: {}", _0)]
    SetWriterFailed(#[fail(cause)] IoError),
}

#[derive(Debug, Default)]
pub struct ResourceWriter {
    dir: String,
    writers: HashMap<ResourceName, LineWriter<File>>,
}

impl ResourceWriter {
    #[inline]
    pub fn new(dir: &str) -> Self {
        ResourceWriter {
            dir: dir.to_owned(),
            ..Default::default()
        }
    }
    #[inline]
    fn add(&mut self, resource: &Resource, writer: LineWriter<File>) {
        self.writers.insert(resource.name(), writer);
    }

    #[inline]
    pub fn init(&mut self, manifest: &Manifest) {
        manifest.status().for_each(|(index, status)| {
            let resource = Resource::new(*index, status.last_file_id());
            match self.build_writer() {
                Ok(writer) => self.add(&resource, writer),
                Err(error) => error!("{}", error),
            }
        })
    }

    #[inline]
    fn build_writer(&self) -> Result<LineWriter<File>, ResourceWriterError> {
        let file = OpenOptions::new()
            .write(true)
            .open(self.dir.to_owned())
            .map_err(ResourceWriterError::SetWriterFailed)?;
        Ok(LineWriter::new(file))
    }
}
