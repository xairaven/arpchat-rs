use thiserror::Error;

#[derive(Error, Debug)]
pub enum NetError {
    #[error("Couldn't send ARP packet")]
    ARPSendFailed,

    #[error("Couldn't serialize ARP packet")]
    ARPSerializeFailed,

    #[error("Couldn't deserialize ARP packet")]
    ARPDeserializeFailed,

    #[error("Couldn't capture packet")]
    CaptureFailed,

    #[error("Error getting channel, might be missing permissions")]
    ChannelGettingError(#[from] std::io::Error),

    #[error("Couldn't parse frame as Ethernet")]
    EthernetParseFailed,

    #[error("Tried to set interface, but interface is already initialized")]
    InterfaceAlreadySet,

    #[error("Invalid interface {0}")]
    InvalidInterface(String),

    #[error("Message too long to send.")]
    MessageTooLong,

    #[error("No MAC Address.")]
    NoMac,

    #[error("Unknown channel type, only ethernet is supported")]
    UnknownChannelType,
}
