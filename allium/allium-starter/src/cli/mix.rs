use super::error::Result;
use derive_new::new;
use std::{path::PathBuf, process::Command};
use tracing::info;

#[derive(new)]
pub struct Mix {
    project_path: PathBuf,
}

impl Mix {
    pub fn mix(&self) -> Result<()> {
        info!("Mix Allium Project");

        let mut backend_path = self.project_path.clone();
        backend_path.push("backend");

        let backend = backend_path.canonicalize()?;
        let mut deps_output = Command::new("mix")
            .arg("deps.get")
            .current_dir(&backend)
            .spawn()?;
        deps_output.wait()?;

        let mut compile_output = Command::new("mix")
            .arg("compile")
            .current_dir(&backend)
            .spawn()?;
        compile_output.wait()?;

        info!("Mix up \u{1f43c}!");

        Ok(())
    }

    pub fn reset_database(&self) -> Result<()> {
        info!("Setup Allium Database");

        let mut backend_path = self.project_path.clone();
        backend_path.push("backend");

        let backend = backend_path.canonicalize()?;
        let mut ecto_drop_output = Command::new("mix")
            .arg("ecto.drop")
            .current_dir(&backend)
            .spawn()?;
        ecto_drop_output.wait()?;

        let mut ecto_create_output = Command::new("mix")
            .arg("ecto.create")
            .current_dir(&backend)
            .spawn()?;
        ecto_create_output.wait()?;

        let mut ecto_migrate_output = Command::new("mix")
            .arg("ecto.migrate")
            .current_dir(&backend)
            .spawn()?;
        ecto_migrate_output.wait()?;

        info!("Database Setup \u{1f43c}!");

        Ok(())
    }

    pub fn register_schemas(&self) -> Result<()> {
        info!("Registering schemas in kafka schemas registry");

        let mut backend_path = self.project_path.clone();
        backend_path.push("backend");

        let backend = backend_path.canonicalize()?;
        let mut output = Command::new("mix")
            .args(["procon.reg.schema", "--all"])
            .current_dir(backend)
            .spawn()?;
        output.wait()?;

        info!("Schemas registered in schema registry \u{1f43c}!");

        Ok(())
    }

    pub fn register_materialize_processors(&self) -> Result<()> {
        info!("Registering materialize processors");

        let mut backend_path = self.project_path.clone();
        backend_path.push("backend");

        let backend = backend_path.canonicalize()?;
        let mut output = Command::new("mix")
            .args(["procon.reg.materialized", "--all"])
            .current_dir(backend)
            .spawn()?;
        output.wait()?;

        info!("Materialize processors registered \u{1f43c}!");

        Ok(())
    }

    pub fn start_server(&self) -> Result<()> {
        info!("Run Allium Project");

        let mut backend_path = self.project_path.clone();
        backend_path.push("backend");

        let backend = backend_path.canonicalize()?;
        let mut output = Command::new("iex")
            .env("KAFKA", "kafka1")
            .args(["-S", "mix", "phx.server"])
            .current_dir(backend)
            .spawn()?;
        output.wait()?;

        info!("Serve up \u{1f43c}!");

        Ok(())
    }
}
