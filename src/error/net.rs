use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetError {
    #[error("No MAC Address.")]
    NoMac,
}
