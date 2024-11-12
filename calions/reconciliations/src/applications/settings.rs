use crate::resources::{ProcessorId, ProcessorTopic};
use config::{Config, ConfigError, File, Value};
use dotenv::{dotenv, vars};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::env;
use std::path::Path;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct ReconciliationData {
    pub debug: Option<String>,
    pub topics: Vec<ProcessorTopic>,
}

#[derive(Deserialize)]
pub struct ReconciliationDataProducer {
    pub debug: Option<String>,
    pub topic: ProcessorTopic,
}

#[derive(Deserialize)]
pub struct ReconciliationProcess {
    pub debug: Option<String>,
    pub topics: Vec<ProcessorTopic>,
}

#[derive(Deserialize)]
pub struct Status {
    pub debug: Option<String>,
    pub topic: ProcessorTopic,
}

#[derive(Deserialize)]
pub struct KafkaConfig {
    pub brokers: Vec<String>,
    pub debug: Option<String>,
    pub group_id: String,
    pub timeout: u64,
}

#[derive(Deserialize)]
pub struct Consumers {
    pub reconciliation_data: ReconciliationData,
    pub reconciliation_process: ReconciliationProcess,
}

#[derive(Deserialize)]
pub struct Producers {
    pub reconciliation_data: ReconciliationDataProducer,
    pub status: Status,
}

#[derive(Deserialize)]
pub struct Kafka {
    pub config: KafkaConfig,
    pub consumers: Consumers,
    pub producers: Producers,
}

#[derive(Deserialize)]
pub struct Settings {
    pub kafka: Kafka,
    pub processor_id: Arc<ProcessorId>,
    pub rust_log: String,
    pub status_heartbeat_interval: u64,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        let config = "config";

        let path = Path::new(config);

        if path.exists() {
            // 1. default
            s.merge(File::with_name(&format!("{}/default", config)))?;

            // 2. environment mode
            let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
            s.merge(File::with_name(&format!("{}/{}", config, env)).required(false))?;

            // 3. local
            s.merge(File::with_name(&format!("{}/local", config)).required(false))?;
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
                let config_value = if value.contains(",") {
                    let mut value: Vec<&str> = value.split(",").collect();
                    value.retain(|v| !v.is_empty());
                    Value::new(uri, value)
                } else {
                    Value::new(uri, value)
                };

                Some((key, config_value))
            })
            .collect();
        for (key, value) in env_vars {
            s.set(&key, value)?;
        }
        if let Ok(rust_log) = env::var("RUST_LOG") {
            s.set("rust_log", rust_log)?;
        }
        s.try_into()
    }

    pub fn get() -> &'static Self {
        static SETTINGS: Lazy<Settings> =
            Lazy::new(|| Settings::new().expect("Unable to initialize settings"));

        &SETTINGS
    }
}
