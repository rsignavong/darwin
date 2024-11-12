use super::{Index, Key, ResourceData, ResourceStatus, MANIFEST_EXTENSION};
use bincode::Error as BincodeError;
use serde::{Deserialize, Serialize};
use std::collections::{hash_map::Iter, HashMap};
use std::fs::OpenOptions;
use std::io::{BufReader, Error as IoError, Read};
use std::path::PathBuf;

#[derive(Debug, Fail)]
pub enum ManifestError {
    #[fail(display = "Failed to deserialize Tesla data: {}", _0)]
    DeserializeFailed(#[fail(cause)] BincodeError),
    #[fail(display = "Failed to load Tesla data: {}", _0)]
    LoadManifestFailed(#[fail(cause)] IoError),
    #[fail(display = "Failed to read Tesla data: {}", _0)]
    ReadManifestFailed(#[fail(cause)] IoError),
    #[fail(display = "Failed to serialize Tesla metadata: {}", _0)]
    SerializeFailed(#[fail(cause)] BincodeError),
}

pub type Size = u16;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Manifest {
    status: HashMap<Index, ResourceStatus>,
    size: Size,
}

impl Manifest {
    #[inline]
    pub fn build_path(dir: &str) -> PathBuf {
        let mut path = PathBuf::from(dir);
        path.push("tesla");
        path.set_extension(MANIFEST_EXTENSION);
        path
    }
    #[inline]
    pub fn load(dir: &str) -> Result<Self, ManifestError> {
        let file = OpenOptions::new()
            .read(true)
            .open(Manifest::build_path(dir))
            .map_err(ManifestError::LoadManifestFailed)?;

        let mut reader = BufReader::new(file);
        let mut bytes: Vec<u8> = Vec::new();

        reader
            .read_to_end(&mut bytes)
            .map_err(ManifestError::ReadManifestFailed)?;

        let manifest: Manifest =
            bincode::deserialize(&bytes).map_err(ManifestError::DeserializeFailed)?;

        Ok(manifest)
    }

    #[inline]
    pub fn new(size: Size) -> impl Fn(ManifestError) -> Result<Self, ManifestError> {
        move |error| {
            if let ManifestError::LoadManifestFailed(io_error) = error {
                debug!("Load manifest failed: {}", io_error);
                warn!("Initialize new Tesla Manifest");
                Ok(Manifest {
                    size,
                    ..Default::default()
                })
            } else {
                Err(error)
            }
        }
    }

    #[inline]
    pub fn get_index(&self, key: &Key) -> Option<(Index, &ResourceStatus)> {
        key.index()
            .and_then(|index| self.status.get(&index).map(move |status| (index, status)))
    }

    #[inline]
    pub fn serialize(&self) -> Result<Vec<u8>, ManifestError> {
        bincode::serialize(self).map_err(ManifestError::SerializeFailed)
    }

    #[inline]
    pub fn size(&self) -> Size {
        self.size
    }
    #[inline]
    pub fn status(&self) -> Iter<Index, ResourceStatus> {
        self.status.iter()
    }

    // #[inline]
    // pub fn add(&mut self, alpha_num: Index) -> Result<BufWriter<File>, ManifestError> {
    //     let (counter, lines) = self.filemap.entry(alpha_num).or_insert((0, 0));
    //     *counter += 1;
    //     let dbfile = OpenOptions::new()
    //         .create(true)
    //         .write(true)
    //         .open(format!(
    //             "{}/{}{}{}",
    //             Settings::get().tesla.data,
    //             alpha_num,
    //             *counter,
    //             extension
    //         ))
    //         .map_err(|err| ManifestError::CreateDatabaseFailed(err))?;

    //     Ok(BufWriter::new(dbfile))
    // }
}
