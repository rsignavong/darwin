use config::{Config as Configurator, ConfigError, File, Value};
use dotenv::{dotenv, vars};
use once_cell::sync::Lazy;
use repository::RepositoryConfig;
use serde::Deserialize;
use std::{env, path::Path};
use web::WebConfig;

#[derive(Deserialize)]
pub struct Config {
    pub name: String,
    pub repository: RepositoryConfig,
    pub rust_log: String,
    pub web: WebConfig,
}

impl Config {
    pub fn new() -> Result<Self, ConfigError> {
        let mut builder = Configurator::builder();
        let config = "config";
        let path = Path::new(config);

        if path.exists() {
            // 1. default
            builder = builder.add_source(File::with_name(&format!("{}/default", config)));

            // 2. environment mode
            let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".to_owned());
            builder =
                builder.add_source(File::with_name(&format!("{}/{}", config, env)).required(false));

            // 3. local
            builder =
                builder.add_source(File::with_name(&format!("{}/local", config)).required(false));
        }

        // 4. environment variables starting with APP_ (e.g. APP_DEBUG = 1)
        // Config::Environment is buggy and doesn't Vector in environment variable
        // this is a little custom hack with dotenv-rs
        dotenv().ok();
        let env_vars: Vec<(String, Value)> = vars()
            .filter_map(|(key, value)| {
                if !key.starts_with("APP__") {
                    return None;
                }

                let parts: Vec<&str> = key.split("__").skip(1).collect();
                let key = parts.join(".").to_lowercase();
                let uri = String::from("the environment");
                let uri = Some(&uri);
                let config_value = if value.contains(',') {
                    let mut value: Vec<&str> = value.split(',').collect();
                    value.retain(|v| !v.is_empty());
                    Value::new(uri, value)
                } else {
                    Value::new(uri, value)
                };

                Some((key, config_value))
            })
            .collect();

        for (key, value) in env_vars {
            builder = builder.set_override(&key, value)?;
        }

        builder.build()?.try_deserialize()
    }

    pub fn get() -> &'static Self {
        static CONFIGS: Lazy<Config> =
            Lazy::new(|| Config::new().expect("Unable to initialize config"));

        &CONFIGS
    }
}
