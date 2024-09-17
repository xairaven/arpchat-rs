// ARP Hardware type. Ethernet - IEEE 802 Numbers
pub const HARDWARE_TYPE_ETHERNET: &[u8] = &[0x00, 0x01];

// ARP Hardware length in bytes. MAC - 6 bytes
pub const HARDWARE_ADDRESS_LENGTH: u8 = 6;

// ARP Operation codes. Reply - 0x1
pub const OPCODE_REQUEST: &[u8] = &[0, 1];
