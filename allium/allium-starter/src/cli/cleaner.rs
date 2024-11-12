use super::error::Result;
use derive_new::new;
use std::{path::PathBuf, process::Command};
use tracing::info;

#[derive(new)]
pub struct Cleaner {
    project_path: PathBuf,
}

impl Cleaner {
    pub fn remove_cache(&self) -> Result<()> {
        info!("Purge Cache");

        let mut output = Command::new("rm")
            .args(["-rf", "cache"])
            .current_dir(&self.project_path)
            .spawn()?;
        output.wait()?;

        info!("Purge up \u{1f43c}!");

        Ok(())
    }

    pub fn remove_generated_source(&self) -> Result<()> {
        info!("Clean up Allium Project");

        let mut output = Command::new("rm")
            .args([
                "-rf",
                ".components.json",
                "backend",
                "frontend",
                "styles",
                "terraform",
            ])
            .current_dir(&self.project_path)
            .spawn()?;
        output.wait()?;

        info!("Clean up \u{1f43c}!");

        Ok(())
    }
}
