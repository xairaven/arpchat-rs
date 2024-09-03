use crate::net::ether_type::EtherType;
use crate::net::ktp;
use pnet::datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::util::MacAddr;
use std::collections::{HashMap, VecDeque};

pub struct Channel {
    src_mac: MacAddr,
    ether_type: EtherType,
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,

    buffer: HashMap<ktp::Id, Vec<Vec<u8>>>,

    recent: VecDeque<ktp::Id>,
}

impl Channel {
    fn from_interface(interface: NetworkInterface) {}
}
