/// ARP Chat is based on the ARP protocol. <br>
/// But, there is a need to use a transport protocol.
/// I chose the name KTP - kognise's transport protocol.

// Custom packet prefix
const PACKET_PREFIX: &[u8] = b"ktp";

pub type Id = [u8; 8];
pub type Tag = u8;
pub type Seq = u8;
pub type Total = u8;

// Packet Header size consists of packet prefix, Id, Tag, Seq and Total fields.
const PACKET_HEADER_SIZE: usize = PACKET_PREFIX.len()
    + size_of::<Id>()
    + size_of::<Seq>()
    + size_of::<Tag>()
    + size_of::<Total>();

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
    fn tag(&self) -> Tag {
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
                let id: Id = data[..size_of::<Id>()].try_into().ok()?;
                let raw_str = smaz::decompress(&data[size_of::<Id>()..]).ok()?;
                let str = String::from_utf8(raw_str).ok()?;
                Some(Packet::Message(id, str))
            },
            1 => Some(Packet::PresenceReq),
            2 => {
                let id: Id = data[..size_of::<Id>()].try_into().ok()?;
                let is_join = data[size_of::<Id>()] > 0;
                let str = String::from_utf8(data[size_of::<Id>() + 1..].to_vec()).ok()?;
                Some(Packet::Presence(id, is_join, str))
            },
            3 => Some(Packet::Disconnect(data.try_into().ok()?)),
            _ => None,
        }
    }
}
