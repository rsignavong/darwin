mod bootstrap;
mod cleaner;
mod component;
mod error;
mod frontend;
mod kafka;
mod mix;
mod postgres;
mod utils;

pub use error::{CliError, Result};

use bootstrap::Bootstrap;
use clap::Parser;
use cleaner::Cleaner;
use component::Component;
use frontend::Frontend;
use kafka::Kafka;
use mix::Mix;
use postgres::Postgres;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::info;
use utils::Utils;

/// Script to speed up allium local development or demo
#[derive(Debug, Parser)]
#[clap(about, author = "Allium Enterprise", version)]
pub struct Cli {
    /// Define project folder to run allium-starter.
    #[clap(
        env,
        long,
        short = 'P',
        parse(from_os_str),
        default_value = "./generated_allium"
    )]
    project_path: PathBuf,

    /// Install bootstrap
    #[clap(long, short = 'b')]
    bootstrap: bool,

    /// Define bootstrap generator path
    #[clap(env, long, parse(from_os_str), default_value = "./bootstrap")]
    bootstrap_path: PathBuf,

    /// Purge cache folder including backend and frontend dependencies
    #[clap(long, short = 'C')]
    cache: bool,

    /// Clean up generated allium source project (.components.json, backend, frontend)
    #[clap(long, short = 'c')]
    clean: bool,

    /// define selected component for other options (e.g. CommandUser.processors.command).
    #[clap(long, parse(try_from_str = Component::from_str), name = "COMPONENT")]
    component: Option<Component>,

    /// Create topics in kafka
    #[clap(long, short = 't')]
    create_topics: bool,

    /// Setup database
    #[clap(long, short = 'd')]
    database: bool,

    /// Dump database, create a dump file. With --dump-file, it will restore that dump
    #[clap(long, short = 'D')]
    dump_commands_data: bool,

    /// Dump database, create a dump file. With --dump-file, it will restore that dump
    #[clap(long, parse(from_os_str))]
    dump_file: Option<PathBuf>,

    /// Define environment
    #[clap(env, long, short = 'e', default_value = "dev")]
    environment: String,

    /// Kafka consumer groups command path
    #[clap(env, long, parse(from_os_str))]
    kafka_consumer_groups_cmd: Option<PathBuf>,

    /// Kafka topics parallel chunk limit creation
    #[clap(env, long, default_value = "50")]
    kafka_topics_chunk_limit: u8,

    /// Kafka topics command path
    #[clap(env, long, parse(from_os_str))]
    kafka_topics_cmd: Option<PathBuf>,

    /// Kafka broker
    #[clap(env, long, default_value = "localhost:9092")]
    kafka_broker: String,

    /// Kafka topic replication factor
    #[clap(env, long, default_value = "3")]
    kafka_topic_replication_factor: u8,

    /// Generate only components and config
    #[clap(long, short = 'g')]
    generate: bool,

    /// Generate all the allium project and processors and setup local.exs
    #[clap(long, short = 'G')]
    generate_all: bool,

    /// Delete duplicate migrations files and force generation of selected COMPONENT
    #[clap(long, short = 'f')]
    force_generate: bool,

    /// Install MFs dependencies and applications dependencies for selected COMPONENT
    #[clap(long, short = 'F')]
    force_generate_and_migrate_compile: bool,

    /// Install MFs dependencies and applications dependencies
    #[clap(long, short = 'I')]
    frontend_install: bool,

    /// Build MFs
    #[clap(long, short = 'B')]
    frontend_build: bool,

    /// Build Web components
    #[clap(long, short = 'W')]
    webcomponents_build: bool,

    /// Define local config path for local.exs
    #[clap(env, long, parse(from_os_str), default_value = "./local.exs")]
    local_config_path: PathBuf,

    /// Pass additional config to merge to bootstrap
    #[clap(long)]
    merge_config: Option<String>,

    /// Mix up the allium project
    #[clap(long, short = 'm')]
    mix: bool,

    /// Define Postgres psql command path
    #[clap(env, long, parse(from_os_str), default_value = "psql")]
    psql_cmd: PathBuf,

    /// Define Postgres pg_dump command path
    #[clap(env, long, parse(from_os_str), default_value = "pg_dump")]
    pg_dump_cmd: PathBuf,

    /// Define Postgres hostname
    #[clap(env, long, default_value = "localhost")]
    pg_host: String,

    /// Define Postgres port
    #[clap(env, long, default_value = "5432")]
    pg_port: u16,

    /// Define Postgres username
    #[clap(env, long, default_value = "postgres")]
    pg_username: String,

    /// Register materialize processors (materialize must be started)
    #[clap(long, short = 'z')]
    register_materialize_processors: bool,

    /// Register schemas in schema registry
    #[clap(long, short = 'a')]
    register_schemas: bool,

    /// Start the server
    #[clap(long, short = 's')]
    server: bool,

    /// Open maestro application in browser
    #[clap(long, short = 'o', name = "MAESTRO_APPLICATION")]
    open: Option<String>,

    /// Convert Yaml File to Json output
    #[clap(long, parse(from_os_str))]
    yaml2json: Option<PathBuf>,
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        info!(
            "Runnning Allium Starter for {}",
            self.project_path.to_string_lossy()
        );
        let project_path =
            self.project_path
                .canonicalize()
                .map_err(|source| CliError::ProjectPath {
                    source,
                    path: self.project_path.to_string_lossy().to_string(),
                })?;

        if self.bootstrap {
            Bootstrap::install(&self.bootstrap_path)?;
        }

        if self.cache {
            Cleaner::new(project_path.clone()).remove_cache()?;
        }

        if self.clean {
            Cleaner::new(project_path.clone()).remove_generated_source()?;
        }

        if self.generate_all {
            Bootstrap::new(project_path.clone())
                .generate_all(&self.local_config_path, self.merge_config.as_deref())?;
        }

        if self.generate {
            Bootstrap::new(project_path.clone())
                .generate_components_and_config(self.merge_config.as_deref())?;
        }

        if self.force_generate {
            let component = self.component.clone().ok_or(CliError::ComponentMissing)?;
            Bootstrap::new(project_path.clone())
                .force_generate(&component, self.merge_config.as_deref())?;
        }

        if self.force_generate_and_migrate_compile {
            let component = self.component.clone().ok_or(CliError::ComponentMissing)?;
            let kafka_consumer_groups_cmd = self
                .kafka_consumer_groups_cmd
                .as_ref()
                .ok_or(CliError::KafkaConsumerGroupsCmdMissing)
                .and_then(|cmd| {
                    cmd.canonicalize()
                        .map_err(|source| CliError::KafkaConsumerGroupsCmdInvalid {
                            source,
                            path: cmd.to_string_lossy().to_string(),
                        })
                })?;
            let kafka_topics_cmd = self
                .kafka_topics_cmd
                .as_ref()
                .ok_or(CliError::KafkaTopicsCmdMissing)
                .and_then(|cmd| {
                    cmd.canonicalize()
                        .map_err(|source| CliError::KafkaTopicsCmdInvalid {
                            source,
                            path: cmd.to_string_lossy().to_string(),
                        })
                })?;
            Bootstrap::new(project_path.clone()).force_generate_and_migrate_compile(
                &self.kafka_broker,
                kafka_consumer_groups_cmd,
                kafka_topics_cmd,
                self.kafka_topic_replication_factor,
                self.environment.clone(),
                &component,
                self.merge_config.as_deref(),
            )?;
        }

        if self.frontend_install {
            Frontend::new(project_path.clone()).install()?;
        }

        if self.frontend_build {
            Frontend::new(project_path.clone()).build()?;
        }

        if self.webcomponents_build {
            Frontend::new(project_path.clone()).build_web_components()?;
        }

        if self.mix {
            Mix::new(project_path.clone()).mix()?;
        }

        if self.database {
            Mix::new(project_path.clone()).reset_database()?;
        }

        if self.create_topics {
            let kafka_topics_cmd = self
                .kafka_topics_cmd
                .as_ref()
                .ok_or(CliError::KafkaTopicsCmdMissing)
                .and_then(|cmd| {
                    cmd.canonicalize()
                        .map_err(|source| CliError::KafkaTopicsCmdInvalid {
                            source,
                            path: cmd.to_string_lossy().to_string(),
                        })
                })?;
            Kafka::new(project_path.clone(), self.kafka_broker.clone()).create_topics(
                kafka_topics_cmd,
                self.kafka_topic_replication_factor,
                self.kafka_topics_chunk_limit,
                self.environment.clone(),
            )?;
        }

        if self.register_schemas {
            Mix::new(project_path.clone()).register_schemas()?;
        }

        if self.register_materialize_processors {
            Mix::new(project_path.clone()).register_materialize_processors()?;
        }

        if self.server {
            Mix::new(project_path.clone()).start_server()?;
        }

        if self.dump_commands_data {
            let psql_cmd = self.psql_cmd.clone();
            let psql_cmd =
                self.psql_cmd
                    .canonicalize()
                    .map_err(move |source| CliError::PsqlCmdInvalid {
                        source,
                        path: psql_cmd.to_string_lossy().to_string(),
                    })?;

            if let Some(dump_file) = &self.dump_file {
                let dump_file =
                    dump_file
                        .canonicalize()
                        .map_err(|source| CliError::PgDumpFileInvalid {
                            source,
                            path: dump_file.to_string_lossy().to_string(),
                        })?;
                Postgres::new(
                    self.environment.clone(),
                    project_path.clone(),
                    self.pg_host.clone(),
                    self.pg_port,
                    self.pg_username.clone(),
                )
                .restore_dump_file(&psql_cmd, &dump_file)?;
            } else {
                let pg_dump_cmd = self.pg_dump_cmd.clone();
                let pg_dump_cmd = self.pg_dump_cmd.canonicalize().map_err(move |source| {
                    CliError::PgDumpCmdInvalid {
                        source,
                        path: pg_dump_cmd.to_string_lossy().to_string(),
                    }
                })?;
                Postgres::new(
                    self.environment.clone(),
                    project_path.clone(),
                    self.pg_host.clone(),
                    self.pg_port,
                    self.pg_username.clone(),
                )
                .backup_commands_data(&psql_cmd, &pg_dump_cmd)?;
            }
        }

        if let Some(maestro_application) = self.open.as_ref() {
            Frontend::new(project_path).open(maestro_application)?;
        }

        if let Some(yaml_file_path) = self.yaml2json.as_ref() {
            Utils::yaml2json(yaml_file_path)?;
        }

        Ok(())
    }
}
