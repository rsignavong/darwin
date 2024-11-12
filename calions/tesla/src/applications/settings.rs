use crate::SETTINGS;
use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;
use std::{env, sync::RwLockReadGuard};

#[derive(Debug, Deserialize)]
pub struct Tesla {
    pub data: String,
    pub file_size: u16,
    pub pool_size: u8,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub log_level: String,
    pub tesla: Tesla,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // 1. default
        s.merge(File::with_name("config/default"))?;

        // 2. environment mode
        let env = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());
        s.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // 3. local
        s.merge(File::with_name("config/local").required(false))?;

        // 4. environment variables starting with APP_ (e.g. APP_DEBUG = 1)
        s.merge(Environment::with_prefix("app"))?;

        s.try_into()
    }

    pub fn get() -> RwLockReadGuard<'static, Self> {
        match SETTINGS.read() {
            Ok(settings) => settings,
            Err(poisoned) => poisoned.into_inner(),
        }
    }
}
