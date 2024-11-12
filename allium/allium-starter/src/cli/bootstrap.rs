use crate::cli::{
    component::Component,
    error::{CliError, Result},
    kafka::Kafka,
};
use derive_new::new;
use heck::{ToKebabCase, ToPascalCase, ToSnakeCase};
use inflector::string::pluralize::to_plural;
use serde::Deserialize;
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::Command,
};
use tracing::info;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootstrapConfigJson {
    database_prefix: String,
    organization: String,
    name: String,
}

impl BootstrapConfigJson {
    pub fn get_database_prefix(&self) -> &str {
        &self.database_prefix
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_organization(&self) -> &str {
        &self.organization
    }
}

static BOOTSTRAP_CONFIG_JSON_PATH: &str = "bootstrap/bootstrap.config.json";

#[derive(new)]
pub struct Bootstrap {
    project_path: PathBuf,
}

impl Bootstrap {
    pub fn install(bootstrap_path: &Path) -> Result<()> {
        info!("Install Bootstrap");

        let bootstrap =
            fs::canonicalize(bootstrap_path).map_err(|source| CliError::BootstrapPath {
                source,
                path: bootstrap_path.to_string_lossy().to_string(),
            })?;
        let mut output = Command::new("npm")
            .args(["i", "-g", "."])
            .current_dir(bootstrap)
            .spawn()?;
        output.wait()?;

        info!("Boot up \u{1f43c}!");

        Ok(())
    }

    pub fn generate_all(&self, local_config_path: &Path, merge_config: Option<&str>) -> Result<()> {
        info!("Generate Project");

        let bootstrap_config_json = Bootstrap::get_bootstrap_config_json(&self.project_path)?;

        let mut args = vec!["system", BOOTSTRAP_CONFIG_JSON_PATH, "-icCf"];
        Bootstrap::append_args(&mut args, merge_config);
        let mut gen_output = Command::new("allium.gen")
            .args(&args)
            .current_dir(&self.project_path)
            .spawn()?;
        gen_output.wait()?;

        let local_exs =
            fs::read_to_string(fs::canonicalize(local_config_path).map_err(|source| {
                CliError::LocalConfigPath {
                    source,
                    path: local_config_path.to_string_lossy().to_string(),
                }
            })?)?;
        let project_local_exs = local_exs
            .replace("allium", &bootstrap_config_json.name.to_snake_case())
            .replace("Allium", &bootstrap_config_json.name.to_pascal_case());

        let mut project_local_exs_path = self.project_path.clone();
        project_local_exs_path.push("backend/config/local.exs");

        let mut local_exs_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&project_local_exs_path)?;
        local_exs_file.write_all(project_local_exs.as_bytes())?;

        info!("Gen up \u{1f43c}!");

        Ok(())
    }

    pub fn generate_components_and_config(&self, merge_config: Option<&str>) -> Result<()> {
        info!("Generate Components and Config");

        let mut args = vec!["system", BOOTSTRAP_CONFIG_JSON_PATH, "-cC"];
        Bootstrap::append_args(&mut args, merge_config);
        let mut gen_output = Command::new("allium.gen")
            .args(["system", BOOTSTRAP_CONFIG_JSON_PATH, "-cC"])
            .current_dir(&self.project_path)
            .spawn()?;
        gen_output.wait()?;

        info!("Gen components & config up \u{1f43c}!");

        Ok(())
    }

    pub fn get_bootstrap_config_json(project_path: &Path) -> Result<BootstrapConfigJson> {
        let mut bootstrap_json_path = project_path.to_path_buf();
        bootstrap_json_path.push(BOOTSTRAP_CONFIG_JSON_PATH);

        let bootstrap_json = bootstrap_json_path.canonicalize()?;
        let mut file = File::open(bootstrap_json)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let bootstrap_config_json: BootstrapConfigJson = serde_json::from_slice(&buffer)?;

        Ok(bootstrap_config_json)
    }

    pub fn force_generate(&self, component: &Component, merge_config: Option<&str>) -> Result<()> {
        info!("Force generate Processor");

        if component
            .get_stack()
            .map(|s| s == "processors")
            .unwrap_or_else(|| false)
        {
            info!("Reseting migrations");

            let type_ = component
                .get_type()
                .ok_or_else(|| CliError::ComponentNotTyped(component.get_raw().to_owned()))?;
            let migration_path = format!(
                "backend/priv/{}/{}_pg",
                to_plural(type_).to_snake_case(),
                to_plural(component.get_entity()).to_snake_case()
            );
            let mut rm_output = Command::new("rm")
                .args(["-rf", &migration_path])
                .current_dir(&self.project_path)
                .spawn()?;
            rm_output.wait()?;

            info!("Migrations done");
        }

        let mut args_pre_generate = vec!["system", BOOTSTRAP_CONFIG_JSON_PATH, "-c"];
        Bootstrap::append_args(&mut args_pre_generate, merge_config);
        let mut pre_gen_output = Command::new("allium.gen")
            .args(&args_pre_generate)
            .current_dir(&self.project_path)
            .spawn()?;
        pre_gen_output.wait()?;

        let mut args = vec![
            "system",
            BOOTSTRAP_CONFIG_JSON_PATH,
            component.get_raw(),
            "-CF",
        ];
        Bootstrap::append_args(&mut args, merge_config);
        let mut force_gen_output = Command::new("allium.gen")
            .args(&args)
            .current_dir(&self.project_path)
            .spawn()?;
        force_gen_output.wait()?;

        info!("Force Gen up \u{1f43c}!");

        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn force_generate_and_migrate_compile(
        &self,
        kafka_broker: &str,
        kafka_consumer_groups_cmd: PathBuf,
        kafka_topics_cmd: PathBuf,
        kafka_topic_replication_factor: u8,
        environment: String,
        component: &Component,
        merge_config: Option<&str>,
    ) -> Result<()> {
        info!("Force generate and migrate/compile Processor");

        if component
            .get_stack()
            .map(|s| s == "processors")
            .unwrap_or_else(|| false)
        {
            info!("Reseting migrations");

            let type_ = to_plural(
                component
                    .get_type()
                    .ok_or_else(|| CliError::ComponentNotTyped(component.get_raw().to_owned()))?,
            )
            .to_snake_case();
            let entity = to_plural(component.get_entity()).to_snake_case();
            let migration_path = format!("backend/priv/{type_}/{entity}_pg");
            let mut rm_output = Command::new("rm")
                .args(["-rf", migration_path.as_str()])
                .current_dir(&self.project_path)
                .spawn()?;
            rm_output.wait()?;
        }

        let mut args_pre_gen = vec!["system", BOOTSTRAP_CONFIG_JSON_PATH, "-c"];
        Bootstrap::append_args(&mut args_pre_gen, merge_config);
        let mut pre_gen_output = Command::new("allium.gen")
            .args(&args_pre_gen)
            .current_dir(&self.project_path)
            .spawn()?;
        pre_gen_output.wait()?;

        let mut args = vec![
            "system",
            BOOTSTRAP_CONFIG_JSON_PATH,
            component.get_raw(),
            "-CF",
        ];
        Bootstrap::append_args(&mut args, merge_config);
        let mut force_gen_output = Command::new("allium.gen")
            .args(&args)
            .current_dir(&self.project_path)
            .spawn()?;
        force_gen_output.wait()?;

        match component.get_stack() {
            Some("processors") => {
                let bootstrap_config_json =
                    Bootstrap::get_bootstrap_config_json(&self.project_path)?;
                let project_name: String = bootstrap_config_json.name.to_pascal_case();

                let kafka = Kafka::new(self.project_path.clone(), kafka_broker.to_owned());

                info!("Creating topic if necessary");
                kafka.create_topic_if_needed(
                    kafka_topics_cmd,
                    kafka_topic_replication_factor,
                    environment,
                    component,
                )?;

                info!("Reseting topics offsets");
                kafka.reset_offsets(kafka_consumer_groups_cmd, component)?;

                info!("Migrate...");
                let type_ =
                    to_plural(component.get_type().ok_or_else(|| {
                        CliError::ComponentNotTyped(component.get_raw().to_owned())
                    })?);
                let entity = to_plural(component.get_entity());
                let repo =
                    format!("{project_name}.Processors.{type_}.{entity}.Repositories.{entity}Pg");

                let mut backend_path = self.project_path.clone();
                backend_path.push("backend");

                let backend = backend_path.canonicalize()?;
                let mut ecto_drop_output = Command::new("mix")
                    .args(["ecto.drop", "-r", &repo])
                    .current_dir(&backend)
                    .spawn()?;
                ecto_drop_output.wait()?;

                let mut ecto_create_output = Command::new("mix")
                    .args(["ecto.create", "-r", &repo])
                    .current_dir(&backend)
                    .spawn()?;
                ecto_create_output.wait()?;

                let mut ecto_migrate_output = Command::new("mix")
                    .args(["ecto.migrate", "-r", &repo])
                    .current_dir(backend)
                    .spawn()?;
                ecto_migrate_output.wait()?;
            }
            Some("microfrontends") => {
                let view_type = component.get_component().ok_or_else(|| {
                    CliError::ComponentInvalid("Missing microfrontend component".to_owned())
                })?;
                let mf_name = format!(
                    "{}-{}",
                    to_plural(component.get_entity()).to_kebab_case(),
                    view_type
                );

                info!("Compile MF {}", mf_name);

                let mut frontend_path = self.project_path.clone();
                frontend_path.push("frontend/allium-system");

                let frontend = frontend_path.canonicalize()?;
                let mut mf_compile_output = Command::new("npm")
                    .args(["run", "build", "--", "-c", &mf_name])
                    .current_dir(frontend)
                    .spawn()?;
                mf_compile_output.wait()?;
            }
            _ => (),
        }

        info!("Force and Migrate/Compile up \u{1f43c}!");

        Ok(())
    }

    fn append_args<'a>(args: &mut Vec<&'a str>, merge_config: Option<&'a str>) {
        if let Some(merge_config) = merge_config {
            let mut config_to_merge = vec!["--merge-config", merge_config];
            args.append(&mut config_to_merge);
        }
    }
}
