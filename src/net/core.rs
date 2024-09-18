use crate::error::net::NetError;
use crate::net::channel::Channel;
use crate::net::commands::NetCommand;
use crate::net::core::NetThreadState::NeedsInitialPresence;
use crate::net::ktp::Packet;
use crate::net::presence::{
    UpdatePresenceKind, HEARTBEAT_INTERVAL, INACTIVE_TIMEOUT, OFFLINE_TIMEOUT,
};
use crate::net::{interface, ktp};
use crate::session_settings;
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
    log::info!("Net thread started.");

    let session_id = ktp::generate_id();
    let mut session_username = String::from(session_settings::INITIAL_USERNAME);

    let mut last_heartbeat = Instant::now();
    let mut online: HashMap<ktp::Id, (Instant, String)> = HashMap::new();
    let mut offline: HashSet<ktp::Id> = HashSet::new();

    let mut state = NetThreadState::NeedsUsername;
    let mut pause_heartbeat = false;

    let mut channel: Channel;

    log::info!("Interface loop started.");
    loop {
        match net_rx.try_recv() {
            Ok(NetCommand::SetInterface { interface_name }) => {
                let result = interface::channel_from_name(interface_name);
                match result {
                    Ok(value) => {
                        channel = value;
                        log::info!("Net channel created");
                        break;
                    },
                    Err(err) => {
                        log::error!("{}", err.to_string());
                        let _ = ui_tx.try_send(UICommand::SendNetError(err));
                        return;
                    },
                }
            },
            Ok(NetCommand::Terminate) => {
                log::info!("Net Command: Terminate before setting interface.");

                return;
            },
            _ => continue,
        }
    }

    log::info!("Net Thread loop started.");
    loop {
        match net_rx.try_recv() {
            Ok(NetCommand::PauseHeartbeat(pause)) => {
                log::info!("Net Command: Pause Heartbeat. Value = {pause}");
                pause_heartbeat = pause
            },
            Ok(NetCommand::SendMessage { message_text }) => {
                log::info!("Net Command: Send Message. Message: {message_text}");

                let result = ui_tx.try_send(UICommand::ShowMessage {
                    id: session_id,
                    username: session_username.clone(),
                    message: message_text.clone(),
                    is_outgoing_message: true,
                });
                if let Err(err) = result {
                    log::error!("{}", err.to_string());
                }

                let result = channel.try_send(Packet::Message {
                    id: session_id,
                    message_text,
                });
                if let Err(err) = result {
                    log::error!("{}", err.to_string());
                    send_net_error_to_ui(&ui_tx, err);
                } else {
                    log::debug!("Net Command: Sent packet!");
                }
            },
            Ok(NetCommand::SetInterface { .. }) => {
                log::error!("{}", NetError::InterfaceAlreadySet.to_string());

                send_net_error_to_ui(&ui_tx, NetError::InterfaceAlreadySet)
            },
            Ok(NetCommand::SetEtherType(ether_type)) => {
                log::info!("Net Command: Set EtherType. Set {}", ether_type);

                channel.set_ether_type(ether_type);
            },
            Ok(NetCommand::Terminate) => {
                log::info!("Net Command: Terminate.");

                let _ = channel.try_send(Packet::Disconnect(session_id));
                break;
            },
            Ok(NetCommand::UpdateUsername(new_username)) => {
                log::info!("Net Command: Update username: {new_username}");

                session_username = new_username;

                if state == NetThreadState::NeedsUsername {
                    if let Err(err) = channel.try_send(Packet::PresenceBroadcastRequest) {
                        log::error!("{}", err.to_string());
                    }
                    state = NeedsInitialPresence;
                }
            },
            Err(_) => {},
        }

        let result_recv_packet = channel.try_recv();
        let packet = match result_recv_packet {
            Ok(value) => value,
            Err(err) => {
                log::error!("Channel recv error: {}", err.to_string());
                continue;
            },
        };
        match packet {
            None => {},
            Some(Packet::Message { id, message_text }) => {
                log::debug!("Channel: Message Packet received.");

                let username = match online.get(&id) {
                    Some((_, username)) => username.clone(),
                    None => session_settings::UNKNOWN_USERNAME.to_string(),
                };

                // Alerting user if there's username in message
                if id != session_id && message_text.contains(&session_username) {
                    let _ = ui_tx.try_send(UICommand::AlertUser);
                }

                let _ = ui_tx.try_send(UICommand::ShowMessage {
                    id,
                    username,
                    message: message_text,
                    is_outgoing_message: false,
                });
            },
            Some(Packet::PresenceBroadcastRequest) => {
                log::debug!("Channel: Presence Broadcast Request received.");

                let is_user_joining = state == NeedsInitialPresence;
                let packet = Packet::PresenceInformation {
                    id: session_id,
                    is_join: is_user_joining,
                    username: session_username.clone(),
                };

                if let Err(e) = channel.try_send(packet) {
                    log::error!("After sending PresenceInformation: {}", e.to_string());
                }
            },
            Some(Packet::PresenceInformation {
                id: some_id,
                is_join,
                username,
            }) => {
                log::debug!("Channel: Presence Information packet received.");

                match online.insert(some_id, (Instant::now(), username.clone())) {
                    Some((_, previous_username)) => {
                        if let Err(err) = ui_tx.try_send(UICommand::PresenceUpdate {
                            id: some_id,
                            username,
                            is_inactive: false,
                            kind: UpdatePresenceKind::UsernameChange {
                                previous_username,
                            },
                        }) {
                            log::error!(
                                "After sending PresenceUpdate packet: {}",
                                err.to_string()
                            );
                        }
                    },
                    None => {
                        if let Err(err) = ui_tx.try_send(UICommand::PresenceUpdate {
                            id: some_id,
                            username,
                            is_inactive: false,
                            kind: if offline.remove(&some_id) || is_join {
                                UpdatePresenceKind::JoinOrReconnect
                            } else {
                                UpdatePresenceKind::Boring
                            },
                        }) {
                            log::error!(
                                "After sending PresenceUpdate packet: {}",
                                err.to_string()
                            );
                        }
                    },
                }

                if some_id == session_id {
                    state = NetThreadState::Ready;
                }
            },
            Some(Packet::Disconnect(some_id)) => {
                log::debug!("Channel: Disconnection packet received.");

                if let Some((_, username)) = online.remove(&some_id) {
                    if let Err(err) = ui_tx.try_send(UICommand::RemovePresence {
                        id: some_id,
                        username,
                    }) {
                        log::error!(
                            "After sending Disconnect packet: {}",
                            err.to_string()
                        );
                    }
                }
            },
        }

        if last_heartbeat.elapsed() > HEARTBEAT_INTERVAL && state == NetThreadState::Ready
        {
            if !pause_heartbeat {
                if let Err(err) = channel.try_send(Packet::PresenceInformation {
                    id: session_id,
                    is_join: false,
                    username: session_username.clone(),
                }) {
                    log::error!(
                        "After sending Heartbeat PresenceInformation: {}",
                        err.to_string()
                    );
                } else {
                    log::debug!("Heartbeat: PresenceInformation packet sent");
                }
            }

            let mut to_remove = vec![];
            for (id, (user_last_heartbeat, username)) in online.iter() {
                if user_last_heartbeat.elapsed() > OFFLINE_TIMEOUT {
                    offline.insert(*id);
                    if let Err(err) = ui_tx.try_send(UICommand::RemovePresence {
                        id: *id,
                        username: username.clone(),
                    }) {
                        log::error!(
                            "After sending Heartbeat RemovePresence: {}",
                            err.to_string()
                        );
                    }
                    to_remove.push(*id);
                } else if last_heartbeat.elapsed() > INACTIVE_TIMEOUT {
                    if let Err(err) = ui_tx.try_send(UICommand::PresenceUpdate {
                        id: *id,
                        username: username.clone(),
                        is_inactive: true,
                        kind: UpdatePresenceKind::Boring,
                    }) {
                        log::error!(
                            "After sending Heartbeat PresenceUpdate: {}",
                            err.to_string()
                        );
                    }
                }
            }

            for id in to_remove {
                online.remove(&id);
            }

            last_heartbeat = Instant::now();
        }
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
