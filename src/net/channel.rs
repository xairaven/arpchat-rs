use crate::error::net::NetError;
use crate::net::ether_type::EtherType;
use crate::net::ktp;
use crate::net::ktp::Packet;
use pnet::datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::util::MacAddr;
use std::collections::{HashMap, VecDeque};
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

    pub fn try_send(&self, packet: Packet) -> Result<(), NetError> {
        todo!()
    }

    fn try_send_part() -> Result<(), NetError> {
        todo!()
    }

    pub fn try_recv(&mut self) -> Result<Option<Packet>, NetError> {
        todo!()
    }
}
