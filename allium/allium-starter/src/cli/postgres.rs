use super::{error::Result, CliError};
use crate::cli::bootstrap::{Bootstrap, BootstrapConfigJson};
use chrono::Utc;
use csv::Reader;
use derive_new::new;
use heck::ToSnakeCase;
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};
use tracing::info;

#[derive(new)]
pub struct Postgres {
    environment: String,
    project_path: PathBuf,
    host: String,
    port: u16,
    username: String,
}

impl Postgres {
    fn build_prefix(&self, bootstrap_config_json: &BootstrapConfigJson) -> Result<String> {
        let prefix = format!(
            "{}{}{}{}c_",
            bootstrap_config_json.get_database_prefix(),
            bootstrap_config_json
                .get_organization()
                .to_lowercase()
                .get(0..1)
                .ok_or(CliError::BootstrapConfigJson)?,
            bootstrap_config_json
                .get_name()
                .to_lowercase()
                .get(0..1)
                .ok_or(CliError::BootstrapConfigJson)?,
            self.environment.get(0..1).ok_or(CliError::Environment)?,
        );

        Ok(prefix)
    }

    pub fn backup_commands_data(&self, psql_cmd: &Path, pg_dump_cmd: &Path) -> Result<()> {
        info!("Dump 'Command' processors data");

        let bootstrap_config_json = Bootstrap::get_bootstrap_config_json(&self.project_path)?;
        let prefix = self.build_prefix(&bootstrap_config_json)?;
        let databases = self.list_databases(psql_cmd, &prefix)?;
        let mut sql: Vec<String> = Vec::new();
        for (i, database) in databases.iter().enumerate() {
            sql.push(format!("\\connect {database}"));
            sql.push(self.dump_database(pg_dump_cmd, database, &prefix)?);
            if databases.len() - 1 != i {
                sql.push("\n".repeat(3));
            }
        }
        let dump = sql.join("\n");

        let mut dump_file_path = self.project_path.clone();
        dump_file_path.push("dumps");

        fs::create_dir_all(dump_file_path.clone())?;

        dump_file_path.push(format!(
            "{}_{}_{}_{}.sql",
            bootstrap_config_json.get_database_prefix(),
            bootstrap_config_json.get_organization().to_snake_case(),
            bootstrap_config_json.get_name().to_snake_case(),
            Utc::now().timestamp_millis()
        ));

        let mut file = File::create(dump_file_path.clone())?;
        file.write_all(dump.as_bytes())?;

        info!(
            "Dump created at {} \u{1f43c}!",
            dump_file_path.to_string_lossy().to_string()
        );

        Ok(())
    }

    fn dump_database(&self, pg_dump_cmd: &Path, database: &str, prefix: &str) -> Result<String> {
        let table = database.replace(prefix, "");
        let pg_dump = Command::new(pg_dump_cmd)
            .args([
                "-h",
                &self.host,
                "-p",
                &self.port.to_string(),
                "-U",
                &self.username,
                "-w",
                database,
                "--table",
                &table,
                "--data-only",
                "--inserts",
            ])
            .output()?;
        let pg_dump_output = String::from_utf8(pg_dump.stdout)
            .map_err(|source| CliError::PgDumpOutput { source })?;

        Ok(pg_dump_output)
    }

    fn list_databases(&self, psql_cmd: &Path, prefix: &str) -> Result<Vec<String>> {
        let db_list = Command::new(psql_cmd)
            .args([
                "-h",
                &self.host,
                "-p",
                &self.port.to_string(),
                "-U",
                &self.username,
                "-w",
                "--list",
                "--csv",
            ])
            .output()?;
        let mut db_csv = Reader::from_reader(&*db_list.stdout);
        let databases: Vec<String> = db_csv
            .records()
            .filter_map(|record| {
                let row = record.ok()?;
                let name = row.get(0)?;
                if name.starts_with(prefix) {
                    Some(name.to_owned())
                } else {
                    None
                }
            })
            .collect();

        Ok(databases)
    }

    pub fn restore_dump_file(&self, psql_cmd: &Path, dump_file: &Path) -> Result<()> {
        let dump_file = dump_file.to_string_lossy().to_string();
        info!("Restore dump file {}", dump_file);

        let mut psql_cmd = Command::new(psql_cmd)
            .args([
                "-h",
                &self.host,
                "-p",
                &self.port.to_string(),
                "-U",
                &self.username,
                "-w",
                "-f",
                &dump_file,
            ])
            .spawn()?;
        psql_cmd.wait()?;

        info!("Dump {} restored \u{1f43c}!", dump_file);

        Ok(())
    }
}
