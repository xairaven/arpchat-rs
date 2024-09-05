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
pub enum Packet {
    Message(Id, String),
    Disconnect(Id),
}

impl Packet {
    pub fn tag(&self) -> Tag {
        match self {
            Packet::Disconnect(_) => 0,
            Packet::Message(_, _) => 1,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        match self {
            Packet::Message(id, msg) => {
                [id as &[u8], &smaz::compress(msg.as_bytes())].concat()
            },
            Packet::Disconnect(id) => id.to_vec(),
        }
    }

    pub fn deserialize(tag: u8, data: &[u8]) -> Option<Self> {
        match tag {
            0 => Some(Packet::Disconnect(data.try_into().ok()?)),
            1 => {
                let id: Id = data[..size_of::<Id>()].try_into().ok()?;
                let raw_str = smaz::decompress(&data[size_of::<Id>()..]).ok()?;
                let str = String::from_utf8(raw_str).ok()?;
                Some(Packet::Message(id, str))
            },
            _ => None,
        }
    }
}
