use crate::net::arp;
use crate::net::ether_type::EtherType;
use pnet::datalink::{DataLinkReceiver, DataLinkSender, NetworkInterface};
use pnet::util::MacAddr;
use std::collections::{HashMap, VecDeque};

pub struct Channel {
    src_mac: MacAddr,
    ether_type: EtherType,
    tx: Box<dyn DataLinkSender>,
    rx: Box<dyn DataLinkReceiver>,

    buffer: HashMap<arp::Id, Vec<Vec<u8>>>,

    recent: VecDeque<arp::Id>,
}

impl Channel {
    fn from_interface(interface: NetworkInterface) {}
}
