use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetError {
    #[error("Invalid interface {0}")]
    InvalidInterface(String),

    #[error("No MAC Address.")]
    NoMac,
}
