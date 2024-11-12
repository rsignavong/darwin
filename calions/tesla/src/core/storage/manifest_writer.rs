use super::Manifest;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Error as IoError, Write};

#[derive(Debug, Fail)]
pub enum ManifestWriterError {
    #[fail(display = "Failed to open existing Tesla metadata: {}", _0)]
    OpenExistingFailed(#[fail(cause)] IoError),
    #[fail(display = "Failed to open write Tesla metadata: {}", _0)]
    OpenWriteFailed(#[fail(cause)] IoError),
    #[fail(display = "Failed to create Tesla metadata: {}", _0)]
    PathCreationFailed(#[fail(cause)] IoError),
    #[fail(display = "Failed to save Tesla metadata: {}", _0)]
    SaveFailed(#[fail(cause)] IoError),
}

#[derive(Debug)]
pub struct ManifestWriter(BufWriter<File>);

impl ManifestWriter {
    #[inline]
    pub fn create_path(
        dir: &str,
    ) -> impl Fn(ManifestWriterError) -> Result<File, ManifestWriterError> + '_ {
        move |_error| {
            std::fs::create_dir_all(dir).map_err(ManifestWriterError::PathCreationFailed)?;

            OpenOptions::new()
                .create(true)
                .write(true)
                .open(Manifest::build_path(dir))
                .map_err(ManifestWriterError::OpenWriteFailed)
        }
    }
    #[inline]
    pub fn new(dir: &str) -> Result<Self, ManifestWriterError> {
        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .open(Manifest::build_path(dir))
            .map_err(ManifestWriterError::OpenExistingFailed)
            .or_else(ManifestWriter::create_path(dir))?;

        Ok(ManifestWriter(BufWriter::new(file)))
    }

    #[inline]
    pub fn save(&mut self, manifest_bincode: &[u8]) -> Result<(), ManifestWriterError> {
        let writer = &mut self.0;
        writer
            .write_all(manifest_bincode)
            .map_err(ManifestWriterError::SaveFailed)?;

        Ok(())
    }
}
