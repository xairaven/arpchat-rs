use crate::error::config::ConfigError;
use crate::net::ether_type::EtherType;
use crate::ui;
use directories::ProjectDirs;
use log::LevelFilter;
use rust_i18n::once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::sync::Mutex;
use std::{env, fs};

pub static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::load()));
const CONFIG_FILENAME: &str = "config.toml";

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub ether_type: Option<EtherType>,
    pub interface_name: Option<String>,
    pub language: Option<String>,
    pub log_filename: Option<String>,
    pub log_level: Option<String>,
    pub username: Option<String>,
}

impl Config {
    pub fn get_log_level(&self) -> Option<LevelFilter> {
        if let Some(log_level_str) = &self.log_level {
            let level = LevelFilter::from_str(log_level_str);

            return level.ok();
        }

        None
    }

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
            fs::create_dir_all(parent_path)
                .map_err(|err| ConfigError::CannotRecreatePath(err.to_string()))?;
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
        let mut current_dir = env::current_dir()
            .map_err(|err| ConfigError::CurrentDirFetchFailed(err.to_string()))?;
        current_dir.push(CONFIG_FILENAME);
        Ok(current_dir)
    }
}

/// Getting username from config.
/// If there's no username in config, using system hostname.
pub fn get_username() -> String {
    let username = CONFIG
        .try_lock()
        .ok()
        .and_then(|locked_config| locked_config.username.clone())
        .filter(|username| !username.is_empty())
        .unwrap_or_else(|| {
            gethostname::gethostname()
                .to_string_lossy()
                .split('.')
                .next()
                .unwrap_or("")
                .to_string()
        });

    ui::dialog::username::normalize_username(&username)
}
