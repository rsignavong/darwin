use super::DATA_EXTENSION;
use std::path::PathBuf;

pub type Index = char;
pub type FileId = u16;
pub type LineId = u16;

#[derive(Debug, Fail)]
pub enum ResourceError {
    #[fail(display = "Resource not found: {}", _0)]
    ResourceNotFound(String),
}

pub type ResourceName = String;

#[derive(Debug)]
pub struct Resource(Index, FileId);

impl Resource {
    #[inline]
    pub fn new(index: Index, file_id: FileId) -> Self {
        Resource(index, file_id)
    }
    #[inline]
    pub fn build_path(&self, dir: &str) -> PathBuf {
        let mut path = PathBuf::from(dir);
        path.push(self.name());
        path.set_extension(DATA_EXTENSION);
        path
    }

    #[inline]
    pub fn name(&self) -> ResourceName {
        format!("{}{}", self.0, self.1)
    }
}
