use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::channel::Channel;
use crate::net::commands::NetCommand;
use crate::net::interface;
use crate::ui::commands::UICommand;
use crossbeam::channel::{Receiver, Sender};

pub fn start(ui_tx: Sender<UICommand>, net_rx: Receiver<NetCommand>) {
    let mut channel: Option<Channel> = None;

    loop {
        if let Ok(NetCommand::SetInterface(interface_name)) = net_rx.try_recv()
        {
            let result = channel_from_interface_name(interface_name);
            if let Err(err) = result {
                ui_tx.send(UICommand::NetError(err)).unwrap();
                continue;
            }
            channel = result.ok();
        }
        if channel.is_none() {
            continue;
        }
        let channel = channel.take().unwrap();

        // TODO: everything lol
    }
}

fn channel_from_interface_name(
    interface_name: String,
) -> Result<Channel, NetError> {
    let interface = interface::usable_sorted()
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
