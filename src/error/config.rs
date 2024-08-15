use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Cannot create config file.")]
    CannotCreateFile(String),

    #[error("Cannot recreate specified path by get_config_path.")]
    CannotRecreatePath(String),

    #[error("Current directory does not exist or there are insufficient permissions to access it.")]
    CurrentDirFetchFailed(String),

    #[error("TOML Serializer failed while saving file.")]
    SerializerFailed(String),
}
