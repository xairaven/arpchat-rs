use crate::error::config::ConfigError;
use crate::net::ether_type::EtherType;
use crate::session_settings;
use directories::ProjectDirs;
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::str;
use std::str::FromStr;
use std::string::ToString;
use std::sync::{LazyLock, Mutex};
use std::{env, fs};

pub static CONFIG: LazyLock<Mutex<Config>> = LazyLock::new(|| Mutex::new(Config::load()));
const CONFIG_FILENAME: &str = "config.toml";

pub const DEFAULT_LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Warn;

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub ether_type: Option<EtherType>,
    pub interface_name: Option<String>,
    pub language: Option<String>,
    pub log_level: Option<String>,
    pub username: Option<String>,
}

impl Config {
    pub fn get_log_level(&self) -> Option<LevelFilter> {
        let level = self
            .log_level
            .as_deref()
            .and_then(|log_level_str| LevelFilter::from_str(log_level_str).ok())
            .unwrap_or(DEFAULT_LOG_LEVEL_FILTER);

        Some(level)
    }

    pub fn get_username(&self) -> Option<String> {
        let mut username = self.username.clone().unwrap_or_else(Self::get_hostname);

        if username.is_empty() {
            username = session_settings::INITIAL_USERNAME.to_string()
        };

        Some(session_settings::normalize_username(&username))
    }

    fn get_hostname() -> String {
        gethostname::gethostname()
            .to_string_lossy()
            .split('.')
            .next()
            .unwrap_or("")
            .to_string()
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

/// Getters with locking.
pub fn lock_get_ether_type() -> EtherType {
    CONFIG
        .try_lock()
        .ok()
        .and_then(|locked_config| locked_config.ether_type)
        .unwrap_or_default()
}

pub fn lock_get_log_level() -> LevelFilter {
    if let Ok(config) = CONFIG.try_lock() {
        if let Some(level) = config.get_log_level() {
            return level;
        }
    }

    DEFAULT_LOG_LEVEL_FILTER
}

pub fn lock_get_username() -> String {
    CONFIG
        .try_lock()
        .ok()
        .and_then(|locked_config| locked_config.get_username())
        .unwrap_or(session_settings::INITIAL_USERNAME.to_string())
}
