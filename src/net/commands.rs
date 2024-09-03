use crate::net::ether_type::EtherType;

pub enum NetCommand {
    SetEtherType(EtherType),
    SetInterface(String),
    Terminate,
    UpdateUsername(String),
}
