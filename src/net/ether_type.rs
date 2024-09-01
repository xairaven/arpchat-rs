use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter};

/// Ethernet types. <br>
/// Details: https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml <br>
/// 0800 -> Internet Protocol version 4 (IPv4) <br>
/// 88B5 -> IEEE Std 802 - Local Experimental Ethertype <br>
/// 88B6 -> IEEE Std 802 - Local Experimental Ethertype
#[derive(
    Default,
    Display,
    EnumIter,
    Serialize,
    Deserialize,
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
)]
pub enum EtherType {
    #[strum(serialize = "Experimental1: 0x88B5")]
    #[default]
    Experimental1,

    #[strum(serialize = "Experimental2: 0x88B6")]
    Experimental2,

    #[strum(serialize = "IPv4: 0x0800")]
    IPv4,
}

impl EtherType {
    pub fn bytes(&self) -> &[u8] {
        match self {
            EtherType::Experimental1 => &[0x88, 0xB5],
            EtherType::Experimental2 => &[0x88, 0xB6],
            EtherType::IPv4 => &[0x08, 0x00],
        }
    }
}
