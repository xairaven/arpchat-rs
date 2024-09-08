use crate::error::net::NetError;
use crate::net::ether_type::EtherType;
use crate::net::{arp, ktp};
use pnet::datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::Packet;
use pnet::util::MacAddr;
use std::collections::{HashMap, VecDeque};
use std::io::ErrorKind;
use std::time::Duration;

pub struct Channel {
    src_mac: MacAddr,
    ether_type: EtherType,
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,

    buffer: HashMap<ktp::Id, Vec<Vec<u8>>>,

    recent: VecDeque<ktp::Id>,
}

impl Channel {
    pub fn from_interface(interface: NetworkInterface) -> Result<Self, NetError> {
        let (tx_ethernet, rx_ethernet) = match pnet::datalink::channel(
            &interface,
            pnet::datalink::Config {
                read_timeout: Some(Duration::from_millis(100)),
                ..Default::default()
            },
        ) {
            Ok(pnet::datalink::Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => return Err(NetError::UnknownChannelType),
            Err(err) => return Err(NetError::ChannelGettingError(err)),
        };

        Ok(Self {
            src_mac: interface.mac.ok_or(NetError::NoMac)?,
            ether_type: EtherType::default(),
            tx: tx_ethernet,
            rx: rx_ethernet,
            buffer: HashMap::new(),
            recent: VecDeque::with_capacity(16),
        })
    }

    pub fn set_ether_type(&mut self, ether_type: EtherType) {
        self.ether_type = ether_type;
    }

    pub fn try_send(&mut self, packet: ktp::Packet) -> Result<(), NetError> {
        let data = packet.serialize();
        let parts: Vec<&[u8]> = data.chunks(ktp::PACKET_DATA_SIZE).collect();

        // Possible bug: need to push .

        if parts.len() - 1 > u8::MAX as usize {
            return Err(NetError::MessageTooLong);
        }

        let total = (parts.len() - 1) as ktp::Total;
        let id: ktp::Id = ktp::generate_id();
        for (seq, part) in parts.into_iter().enumerate() {
            self.try_send_part(
                packet.tag(),
                seq as ktp::Seq,
                total as ktp::Total,
                id as ktp::Id,
                part,
            )?;
        }

        Ok(())
    }

    fn try_send_part(
        &mut self, tag: ktp::Tag, seq: ktp::Seq, total: ktp::Total, id: ktp::Id,
        part: &[u8],
    ) -> Result<(), NetError> {
        let data = &[ktp::PACKET_PREFIX, &[tag, seq, total], &id, part].concat();

        // The length of the data must fit in a u8. This should also
        // guarantee that we'll be inside the MTU.
        debug_assert!(
            data.len() <= u8::MAX as usize,
            "Part data is too large ({} > {})",
            data.len(),
            u8::MAX
        );

        let arp_bytes = [
            arp::HARDWARE_TYPE_ETHERNET,
            self.ether_type.bytes(),
            &[arp::HARDWARE_ADDRESS_LENGTH, data.len() as u8],
            arp::OPCODE_REQUEST,
            &self.src_mac.octets(), // Sender hardware address
            data,                   // Sender protocol address
            &[0; 6],                // Target hardware address
            data,                   // Target protocol address
        ]
        .concat();

        let mut ethernet_bytes = vec![0; 14 + arp_bytes.len()];
        let mut ethernet_frame = MutableEthernetPacket::new(&mut ethernet_bytes[..])
            .ok_or(NetError::ARPSerializeFailed)?;
        ethernet_frame.set_destination(MacAddr::broadcast());
        ethernet_frame.set_source(self.src_mac);
        ethernet_frame.set_ethertype(EtherTypes::Arp);
        ethernet_frame.set_payload(&arp_bytes);

        match self.tx.send_to(ethernet_frame.packet(), None) {
            Some(Ok(())) => Ok(()),
            _ => Err(NetError::ARPSendFailed),
        }
    }

    pub fn try_recv(&mut self) -> Result<Option<ktp::Packet>, NetError> {
        let packet = match self.rx.next() {
            Ok(packet) => packet,
            Err(e) => {
                return if e.kind() == ErrorKind::TimedOut {
                    Ok(None)
                } else {
                    Err(NetError::CaptureFailed)
                }
            },
        };
        let packet = match EthernetPacket::new(packet) {
            Some(packet) => packet,
            None => return Ok(None),
        };

        // Early filter for packets that aren't relevant.
        if packet.get_ethertype() != EtherTypes::Arp
            || &packet.payload()[6..8] != arp::OPCODE_REQUEST
            || &packet.payload()[..2] != arp::HARDWARE_TYPE_ETHERNET
            || packet.payload()[4] != arp::HARDWARE_ADDRESS_LENGTH
        {
            return Ok(None);
        }

        let data_len = packet.payload()[5] as usize;
        let data = &packet.payload()[14..14 + data_len];
        if !data.starts_with(ktp::PACKET_PREFIX) {
            return Ok(None);
        }

        if let &[tag, seq, total, ref inner @ ..] = &data[ktp::PACKET_PREFIX.len()..] {
            // TODO: Deserializing...

            todo!()
        } else {
            Ok(None)
        }
    }
}
