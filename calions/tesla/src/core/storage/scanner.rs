use super::{Manifest, Resource, ResourceData, ResourceName};
use bstr::ByteSlice;
use memmap::Mmap;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Error as IoError;

#[derive(Debug, Fail)]
pub enum ScannerError {
    #[fail(display = "Failed to create Memory mapping to file: {}", _0)]
    MemmapFailed(String),
    #[fail(display = "Failed to load resource data: {}", _0)]
    LoadResourceFailed(#[fail(cause)] IoError),
}

#[derive(Debug, Default)]
pub struct Scanner {
    dir: String,
    resources: HashMap<ResourceName, Mmap>,
}

impl Scanner {
    #[inline]
    pub fn new(dir: &str) -> Self {
        Scanner {
            dir: dir.to_owned(),
            ..Default::default()
        }
    }

    #[inline]
    pub fn init(&mut self, manifest: &Manifest) {
        manifest.status().for_each(|(index, status)| {
            (0..=status.last_file_id()).for_each(|file_id| {
                let resource = Resource::new(*index, file_id);
                match self.mmap_resource(&resource) {
                    Ok(mmap) => self.add(&resource, mmap),
                    Err(error) => error!("{}", error),
                }
            });
        });
    }

    #[inline]
    pub fn scan(&self, resource: &Resource, key: &Key) -> ResourceData {
        self.resources.get(&resource.name()).and_then(|mmap| {
            mmap[..].find(key.bytes()).and_then(move |offset| {
                let value_offset = offset + key.len() + 1;
                let value = String::from_utf8(mmap[value_offset..value_offset + 26].to_vec())
                    .map()
                    .ok();
                ResourceData::new(key.clone(), value)
            })
        })
    }

    #[inline]
    fn add(&mut self, resource: &Resource, mmap: Mmap) {
        self.resources.insert(resource.name(), mmap);
    }

    #[inline]
    fn mmap_resource(&self, resource: &Resource) -> Result<Mmap, ScannerError> {
        let file = OpenOptions::new()
            .read(true)
            .open(resource.build_path(&self.dir))
            .map_err(ScannerError::LoadResourceFailed)?;

        let mmap = unsafe {
            Mmap::map(&file).map_err(|_err| ScannerError::MemmapFailed(resource.name()))?
        };
        Ok(mmap)
    }
}
