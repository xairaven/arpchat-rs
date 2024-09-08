use crate::error::net::NetError;
use crate::net::channel::Channel;
use crate::net::commands::NetCommand;
use crate::net::core::NetThreadState::NeedsInitialPresence;
use crate::net::ktp::Packet;
use crate::net::presence::UpdatePresenceKind;
use crate::net::{interface, ktp};
use crate::ui::commands::UICommand;
use crossbeam::channel::{Receiver, Sender, TrySendError};
use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum NetThreadState {
    NeedsUsername,
    NeedsInitialPresence,
    Ready,
}

pub fn start(ui_tx: Sender<UICommand>, net_rx: Receiver<NetCommand>) {
    let session_id = ktp::generate_id();
    let mut local_username = String::new();

    let mut last_heartbeat = Instant::now();
    let mut online: HashMap<ktp::Id, (Instant, String)> = HashMap::new();
    let mut offline: HashSet<ktp::Id> = HashSet::new();

    let mut state = NetThreadState::NeedsUsername;
    let mut pause_heartbeat = false;

    let mut channel: Option<Channel> = None;

    loop {
        if let Ok(NetCommand::SetInterface { interface_name }) = net_rx.try_recv() {
            let result = interface::channel_from_name(interface_name);
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
            Ok(NetCommand::PauseHeartbeat(pause)) => pause_heartbeat = pause,
            Ok(NetCommand::SendMessage { message_text }) => {
                ui_tx
                    .try_send(UICommand::ShowMessage {
                        id: session_id,
                        username: local_username.clone(),
                        message: message_text.clone(),
                    })
                    .unwrap();

                let result = channel.try_send(Packet::Message {
                    id: session_id,
                    message_text,
                });
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
                let _ = channel.try_send(Packet::Disconnect(session_id));
                break;
            },
            Ok(NetCommand::UpdateUsername(new_username)) => {
                local_username = new_username;
                // TODO: idk
                if state == NetThreadState::NeedsUsername {
                    // ..
                    state = NeedsInitialPresence;
                }
            },
            Err(_) => {},
        }

        let result_recv_packet = channel.try_recv();
        if let Err(err) = result_recv_packet {
            send_net_error_to_ui(&ui_tx, err);
            break;
        }
        let packet = result_recv_packet.unwrap();
        match packet {
            None => {},
            Some(Packet::Message { id, message_text }) => {
                let username = match online.get(&id) {
                    Some((_, username)) => username.clone(),
                    None => "Unknown".to_string(),
                };

                // Alerting user if there's username in message
                if id != session_id && message_text.contains(&local_username) {
                    let _ = ui_tx.try_send(UICommand::AlertUser);
                }

                ui_tx
                    .try_send(UICommand::ShowMessage {
                        id,
                        username,
                        message: message_text,
                    })
                    .unwrap();
            },
            Some(Packet::PresenceBroadcastRequest) => {
                let is_user_joining = state == NeedsInitialPresence;
                let packet = Packet::PresenceInformation {
                    id: session_id,
                    is_join: is_user_joining,
                    username: local_username.clone(),
                };

                channel.try_send(packet).unwrap();
            },
            Some(Packet::PresenceInformation {
                id: some_id,
                is_join,
                username,
            }) => {
                match online.insert(some_id, (Instant::now(), username.clone())) {
                    Some((_, previous_username)) => ui_tx
                        .try_send(UICommand::PresenceUpdate {
                            id: some_id,
                            username,
                            is_inactive: false,
                            kind: UpdatePresenceKind::UsernameChange {
                                previous_username,
                            },
                        })
                        .unwrap(),
                    None => ui_tx
                        .try_send(UICommand::PresenceUpdate {
                            id: some_id,
                            username,
                            is_inactive: false,
                            kind: if offline.remove(&some_id) || is_join {
                                UpdatePresenceKind::JoinOrReconnect
                            } else {
                                UpdatePresenceKind::Boring
                            },
                        })
                        .unwrap(),
                }

                if some_id == session_id {
                    state = NetThreadState::Ready;
                }
            },
            Some(Packet::Disconnect(some_id)) => {
                if let Some((_, username)) = online.remove(&some_id) {
                    ui_tx
                        .try_send(UICommand::RemovePresence {
                            id: some_id,
                            username,
                        })
                        .unwrap();
                }
            },
        }

        // TODO: Something related with heartbeat
    }
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
