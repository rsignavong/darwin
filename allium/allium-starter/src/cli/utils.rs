use super::error::Result;
use std::{
    fs::File,
    io::{self, Read, Write},
    path::Path,
};

pub struct Utils;

impl Utils {
    pub fn yaml2json(yaml_file_path: &Path) -> Result<()> {
        let yaml_file = yaml_file_path.canonicalize()?;
        let mut file = File::open(yaml_file)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let json = serde_yaml::from_slice::<serde_json::Value>(&buffer)?;
        let res = serde_json::to_string(&json)?;

        io::stdout().write_all(res.as_bytes())?;

        Ok(())
    }
}
