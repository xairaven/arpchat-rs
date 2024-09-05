use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::channel::Channel;
use crate::net::commands::NetCommand;
use crate::net::ktp::Packet;
use crate::net::{interface, ktp};
use crate::ui::commands::UICommand;
use crossbeam::channel::{Receiver, Sender, TrySendError};

pub fn start(ui_tx: Sender<UICommand>, net_rx: Receiver<NetCommand>) {
    let mut local_username = String::new();
    let mut channel: Option<Channel> = None;

    loop {
        if let Ok(NetCommand::SetInterface { interface_name }) = net_rx.try_recv() {
            let result = channel_from_interface_name(interface_name);
            if let Err(err) = result {
                send_net_error_to_ui(&ui_tx, err);
                continue;
            }
            channel = result.ok();
        }
        if channel.is_none() {
            continue;
        }
        let mut channel = channel.take().unwrap();

        match net_rx.try_recv() {
            Ok(NetCommand::SendMessage { message_text }) => {
                let local_id = ktp::generate_id();

                ui_tx
                    .try_send(UICommand::ShowMessage {
                        id: local_id,
                        username: local_username.clone(),
                        message: message_text.clone(),
                    })
                    .unwrap();

                let result = channel.try_send(Packet::Message(local_id, message_text));
                if let Err(err) = result {
                    send_net_error_to_ui(&ui_tx, err);
                }
            },
            Ok(NetCommand::SetInterface { .. }) => {
                send_net_error_to_ui(&ui_tx, NetError::InterfaceAlreadySet)
            },
            Ok(NetCommand::SetEtherType(ether_type)) => {
                channel.set_ether_type(ether_type);
            },
            Ok(NetCommand::Terminate) => {
                let _ = channel.try_send(Packet::Disconnect(ktp::generate_id()));
                break;
            },
            Ok(NetCommand::UpdateUsername(new_username)) => {
                local_username = new_username;
            },
            Err(_) => {},
        }

        // TODO: everything lol
    }
}

fn channel_from_interface_name(interface_name: String) -> Result<Channel, NetError> {
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

fn send_net_error_to_ui(ui_tx: &Sender<UICommand>, err: NetError) {
    let result = ui_tx.try_send(UICommand::SendNetError(err));

    if let Err(err) = result {
        match err {
            TrySendError::Full(_) => {
                // Channel can't be full, because it is unbounded
            },
            TrySendError::Disconnected(_) => {
                panic!("Channel disconnected.")
            },
        }
    }
}
