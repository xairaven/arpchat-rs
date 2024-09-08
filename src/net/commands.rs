use crate::net::ether_type::EtherType;

pub enum NetCommand {
    PauseHeartbeat(bool),
    SendMessage { message_text: String },
    SetEtherType(EtherType),
    SetInterface { interface_name: String },
    Terminate,
    UpdateUsername(String),
}
