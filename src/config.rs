use crate::error::config::ConfigError;
use directories::ProjectDirs;
use rust_i18n::once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str;
use std::string::ToString;
use std::sync::Mutex;
use std::{env, fs};

pub static CONFIG: Lazy<Mutex<Config>> =
    Lazy::new(|| Mutex::new(Config::load()));
const CONFIG_FILENAME: &str = "config.toml";

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub username: Option<String>,
    pub interface_name: Option<String>,
}

impl Config {
    pub fn load() -> Self {
        match Self::get_config_path() {
            Ok(path) => {
                let data = fs::read(path).unwrap_or_default();
                let data: &str = str::from_utf8(&data).unwrap_or_default();
                toml::from_str(data).unwrap_or_default()
            },
            Err(_) => Default::default(),
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let data = toml::to_string(&self)
            .map_err(|err| ConfigError::SerializerFailed(err.to_string()))?;

        let path = Self::get_config_path()?;
        if let Some(parent_path) = path.parent() {
            fs::create_dir_all(parent_path).map_err(|err| {
                ConfigError::CannotRecreatePath(err.to_string())
            })?;
        }
        fs::write(path, data)
            .map_err(|err| ConfigError::CannotCreateFile(err.to_string()))?;

        Ok(())
    }

    fn get_config_path() -> Result<PathBuf, ConfigError> {
        let dirs = ProjectDirs::from("dev", "xairaven", "arpchat-rs");
        match dirs {
            None => Ok(Self::get_current_directory()?),
            Some(value) => Ok(value.config_dir().join(CONFIG_FILENAME)),
        }
    }

    fn get_current_directory() -> Result<PathBuf, ConfigError> {
        let mut current_dir = env::current_dir().map_err(|err| {
            ConfigError::CurrentDirFetchFailed(err.to_string())
        })?;
        current_dir.push(CONFIG_FILENAME);
        Ok(current_dir)
    }
}
