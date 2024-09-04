use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetError {
    #[error("Tried to set interface, but interface is already initialized")]
    InterfaceAlreadySet,

    #[error("Invalid interface {0}")]
    InvalidInterface(String),

    #[error("No MAC Address.")]
    NoMac,
}
