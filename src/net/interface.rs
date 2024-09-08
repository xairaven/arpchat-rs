use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::channel::Channel;
use pnet::datalink::NetworkInterface;

/// Usable interfaces. <br>
/// Necessary: Presence of MAC address. <br>
/// Necessary: Presence of IP (at least 1).
pub fn usable_sorted() -> Vec<NetworkInterface> {
    let mut interfaces: Vec<NetworkInterface> = pnet::datalink::interfaces()
        .into_iter()
        .filter(|interface| interface.mac.is_some() && !interface.ips.is_empty())
        .collect();

    interfaces.sort_by_key(|interface| interface.ips.len());
    interfaces.reverse();

    interfaces
}

pub fn channel_from_name(interface_name: String) -> Result<Channel, NetError> {
    let interface = usable_sorted()
        .into_iter()
        .find(|interface| interface.name.eq(&interface_name))
        .ok_or(NetError::InvalidInterface(interface_name))?;

    let mut channel = Channel::from_interface(interface)?;
    if let Ok(config) = CONFIG.try_lock() {
        if let Some(ether_type) = config.ether_type {
            channel.set_ether_type(ether_type);
        }
    }

    Ok(channel)
}
