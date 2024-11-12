use super::errors::TeslaError;
use super::storage::ManifestWriter;
use super::storage::Resource;
use super::storage::ResourceData;
use super::storage::ResourceWriter;
use super::storage::Scanner;
use super::storage::{Key, Value};
use super::storage::{Manifest, Size};
use rayon::prelude::*;

#[derive(Debug)]
pub struct Tesla {
    manifest: Manifest,
    manifest_writer: ManifestWriter,
    scanner: Scanner,
    resource_writer: ResourceWriter,
}

impl Tesla {
    #[inline]
    pub fn init(dir: &str, size: Size) -> Result<Self, TeslaError> {
        info!("Starting Tesla database");
        let manifest: Manifest = Manifest::load(dir).or_else(Manifest::new(size))?;
        let mut manifest_writer: ManifestWriter = ManifestWriter::new(dir)?;
        manifest_writer.save(&manifest.serialize()?)?;
        let mut scanner: Scanner = Scanner::new(dir);
        scanner.init(&manifest);
        let mut resource_writer: ResourceWriter = ResourceWriter::new(dir);
        resource_writer.init(&manifest);

        Ok(Tesla {
            manifest,
            manifest_writer,
            scanner,
            resource_writer,
        })
    }

    pub fn create(&self, data: &str) -> Result<Value, TeslaError> {
        // 1. get first letter of data
        let key = Key::new(data);
        if let Some((index, status)) = self.manifest.get_index(&key) {
            (0..=status.last_file_id())
                .into_par_iter()
                .map(move |file_id| {
                    let resource = Resource::new(index, file_id);
                    self.scanner.scan(&resource, &key);
                })
                .collect();
        }
        // 2. get letter and FileId from manifest
        // 3. if not exists
        //      a. create file with manifest
        //      b. save manifest with manifest writer
        //      c. write data + value with resource writer
        //      d. update manifest counter for letter
        //      e. save manifest with manifest writer
        //      f. return value
        //    else
        // 3. get all resources from scanner
        // 4. for each resource
        //      find data
        //          if not exists
        //              a. save manifest with manifest writer
        //              b. write data + value with resource writer
        //              c. update manifest counter for letter
        //              d. save manifest with manifest writer
        //              c. return value
        //          else
        //              return value
    }

    pub fn find(&mut self, data: &str) -> Option<Value> {
        // 1. get first letter of data
        // 2. get letter and FileId from manifest
        // 3. if not exists
        //      return none
        //    else
        // 3. get all resources from scanner
        // 4. for each resource
        //      find data
        //          if not exists
        //              none
        //          else
        //              return value
    }

    pub fn find_all(&mut self, data: &[String]) -> Vec<(Key, Value)> {
        // 1. get first letter of data
        // 2. get letter and FileId from manifest
        // 3. if not exists
        //      return none
        //    else
        // 3. get all resources from scanner
        // 4. for each resource
        //      find data
        //          if not exists
        //              none
        //          else
        //              return value
    }
}
