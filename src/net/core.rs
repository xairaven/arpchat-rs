use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::channel::Channel;
use crate::net::commands::NetCommand;
use crate::net::ktp::Packet;
use crate::net::{interface, ktp};
use crate::ui::commands::UICommand;
use crossbeam::channel::{Receiver, Sender, TrySendError};
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};
use crate::net::core::NetThreadState::NeedsInitialPresence;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);
const INACTIVE_TIMEOUT: Duration = Duration::from_secs(6);
const OFFLINE_TIMEOUT: Duration = Duration::from_secs(12);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NetThreadState {
    NeedsUsername,
    NeedsInitialPresence,
    Ready,
}

pub fn start(ui_tx: Sender<UICommand>, net_rx: Receiver<NetCommand>) {
    let user_id = ktp::generate_id();
    let mut local_username = String::new();

    let mut last_heartbeat = Instant::now();
    let mut online: HashMap<ktp::Id, (Instant, String)> = HashMap::new();
    let mut offline: HashSet<ktp::Id> = HashSet::new();

    let mut state = NetThreadState::NeedsUsername;
    let mut pause_heartbeat = false;

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
                ui_tx
                    .try_send(UICommand::ShowMessage {
                        id: user_id,
                        username: local_username.clone(),
                        message: message_text.clone(),
                    })
                    .unwrap();

                let result = channel.try_send(Packet::Message { id: user_id, message_text });
                if let Err(err) = result {
                    send_net_error_to_ui(&ui_tx, err);
                }
            }
            Ok(NetCommand::SetInterface { .. }) => {
                send_net_error_to_ui(&ui_tx, NetError::InterfaceAlreadySet)
            }
            Ok(NetCommand::SetEtherType(ether_type)) => {
                channel.set_ether_type(ether_type);
            }
            Ok(NetCommand::Terminate) => {
                let _ = channel.try_send(Packet::Disconnect(user_id));
                break;
            }
            Ok(NetCommand::UpdateUsername(new_username)) => {
                local_username = new_username;
                // TODO: idk
                if state == NetThreadState::NeedsUsername {
                    // ..
                    state = NetThreadState::NeedsInitialPresence;
                }
            }
            Err(_) => {}
        }

        let result_recv_packet = channel.try_recv();
        if let Err(err) = result_recv_packet {
            send_net_error_to_ui(&ui_tx, err);
            break;
        }
        let packet = result_recv_packet.unwrap();
        match packet {
            None => {}
            Some(Packet::Message { id, message_text }) => {
                // Alerting user if there's username in message
                if id != user_id && message_text.contains(&local_username) {
                    let _ = ui_tx.try_send(UICommand::AlertUser);
                }

                // TODO: Username related thing
                ui_tx
                    .try_send(UICommand::ShowMessage {
                        id,
                        username: "IDK".to_string(),
                        message: message_text,
                    })
                    .unwrap();
            }
            Some(Packet::PresenceBroadcastRequest) => {
                let is_user_joining = state == NeedsInitialPresence;
                let packet = Packet::PresenceInformation {
                    id: user_id,
                    is_join: is_user_joining,
                    username: local_username.clone(),
                };

                channel.try_send(packet).unwrap();
            },
            Some(Packet::PresenceInformation { id, is_join, username }) => {
                // TODO: Update online info
            },
            Some(Packet::Disconnect(_)) => {
                // TODO: Something related with online panel
            },
        }

        // TODO: Something related with heartbeat
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
            }
            TrySendError::Disconnected(_) => {
                panic!("Channel disconnected.")
            }
        }
    }
}
