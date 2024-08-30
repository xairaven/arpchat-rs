pub mod interface;
pub mod ether_type;

const ARP_HTYPE_ETHERNET: &[u8] = &[0x00, 0x01]; // ARP Hardware type. Ethernet - IEEE 802 Numbers
const ARP_HADRR_LENGTH: u8 = 6; // ARP Hardware length in bytes. MAC - 6 bytes

const ARP_OPCODE_REQUEST: u8 = 0x0; // ARP Request Operation Code
const ARP_OPCODE_REPLY: u8 = 0x1; // ARP Reply Operation Code


// Custom packet prefix
const PACKET_PREFIX: &[u8] = b"xai";