// ARP Hardware type. Ethernet - IEEE 802 Numbers
const HARDWARE_TYPE_ETHERNET: &[u8] = &[0x00, 0x01];

// ARP Hardware length in bytes. MAC - 6 bytes
const HARDWARE_ADDRESS_LENGTH: u8 = 6;

// ARP Operation codes. Request - 0x0, Reply - 0x1
const OPCODE_REQUEST: u8 = 0x0;
const OPCODE_REPLY: u8 = 0x1;
