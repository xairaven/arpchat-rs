use serde::{Deserialize, Serialize};

/// Ethernet types. <br>
/// Details: https://www.iana.org/assignments/ieee-802-numbers/ieee-802-numbers.xhtml <br>
/// 0800 -> Internet Protocol version 4 (IPv4) <br>
/// 88B5 -> IEEE Std 802 - Local Experimental Ethertype <br>
/// 88B6 -> IEEE Std 802 - Local Experimental Ethertype
#[derive(Default, Serialize, Deserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum EtherType {
    #[default]
    Experimental1,
    Experimental2,
    IPv4
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