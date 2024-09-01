// ARP Hardware type. Ethernet - IEEE 802 Numbers
const HARDWARE_TYPE_ETHERNET: &[u8] = &[0x00, 0x01];

// ARP Hardware length in bytes. MAC - 6 bytes
const HARDWARE_ADDRESS_LENGTH: u8 = 6;

// ARP Operation codes. Request - 0x0, Reply - 0x1
const OPCODE_REQUEST: u8 = 0x0;
const OPCODE_REPLY: u8 = 0x1;

// Custom packet prefix
const PACKET_PREFIX: &[u8] = b"xai";

// Id consists of 8 bytes.
const ID_SIZE_BYTES: usize = 8;
type Id = [u8; ID_SIZE_BYTES];

// Packet Header size consists of packet prefix, id size, Tag, Seq and total
const PACKET_HEADER_SIZE: usize = PACKET_PREFIX.len() + ID_SIZE_BYTES + 3;

// Possible packet payload size
const PACKET_DATA_SIZE: usize = (u8::MAX as usize) - PACKET_HEADER_SIZE;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Packet {
    Message(Id, String),
    PresenceReq,
    Presence(Id, bool, String),
    Disconnect(Id),
}

impl Packet {
    fn tag(&self) -> u8 {
        match self {
            Packet::Message(_, _) => 0,
            Packet::PresenceReq => 1,
            Packet::Presence(_, _, _) => 2,
            Packet::Disconnect(_) => 3,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        match self {
            Packet::Message(id, msg) => {
                [id as &[u8], &smaz::compress(msg.as_bytes())].concat()
            },
            Packet::PresenceReq => vec![],
            Packet::Presence(id, is_join, str) => {
                [id as &[u8], &[*is_join as u8], str.as_bytes()].concat()
            },
            Packet::Disconnect(id) => id.to_vec(),
        }
    }

    fn deserialize(tag: u8, data: &[u8]) -> Option<Self> {
        match tag {
            0 => {
                let id: Id = data[..ID_SIZE_BYTES].try_into().ok()?;
                let raw_str = smaz::decompress(&data[ID_SIZE_BYTES..]).ok()?;
                let str = String::from_utf8(raw_str).ok()?;
                Some(Packet::Message(id, str))
            },
            1 => Some(Packet::PresenceReq),
            2 => {
                let id: Id = data[..ID_SIZE_BYTES].try_into().ok()?;
                let is_join = data[ID_SIZE_BYTES] > 0;
                let str = String::from_utf8(data[ID_SIZE_BYTES + 1..].to_vec())
                    .ok()?;
                Some(Packet::Presence(id, is_join, str))
            },
            3 => Some(Packet::Disconnect(data.try_into().ok()?)),
            _ => None,
        }
    }
}
