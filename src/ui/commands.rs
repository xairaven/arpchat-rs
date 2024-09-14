use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::commands::NetCommand;
use crate::net::ether_type::EtherType;
use crate::net::ktp;
use crate::net::presence::UpdatePresenceKind;
use crate::{config, ui};
use chrono::Timelike;
use crossbeam::channel::Sender;
use cursive::backends::crossterm::crossterm::style::Stylize;
use cursive::views::{LinearLayout, NamedView, TextView};
use cursive::Cursive;

pub enum UICommand {
    AlertUser,

    SendNetError(NetError),

    SendMessage {
        message_text: String,
    },

    SetEtherType(EtherType),
    SetInterface(String),
    SetLanguage(String),
    SetUsername(String),

    ShowMessage {
        id: ktp::Id,
        username: String,
        message: String,
        is_outgoing_message: bool,
    },

    PresenceUpdate {
        id: ktp::Id,
        username: String,
        is_inactive: bool,
        kind: UpdatePresenceKind,
    },

    RemovePresence {
        id: ktp::Id,
        username: String,
    },
}

pub fn alert_user() {
    // Ringing dat bell
    use std::io::{stdout, Write};
    print!("\x07");
    let _ = stdout().flush();
}

pub fn send_message(
    message_text: String, siv: &mut Cursive, net_tx: &Sender<NetCommand>,
) {
    if message_text.eq("/offline") {
        let _ = net_tx.try_send(NetCommand::PauseHeartbeat(true));
    } else if message_text.eq("/online") {
        let _ = net_tx.try_send(NetCommand::PauseHeartbeat(false));
    } else if !message_text.is_empty() {
        let result = net_tx.try_send(NetCommand::SendMessage { message_text });

        if let Err(err) = result {
            ui::dialog::error::show(siv, err);
        }
    }
}

pub fn set_ether_type(
    ether_type: EtherType, siv: &mut Cursive, net_tx: &Sender<NetCommand>,
) {
    let result = net_tx.try_send(NetCommand::SetEtherType(ether_type));

    if let Err(err) = result {
        ui::dialog::error::show_try_again(siv, err.to_string());
        return;
    }

    if let Ok(mut config) = CONFIG.try_lock() {
        config.ether_type = Some(ether_type);
        config.save().unwrap_or_default();
    }
}

pub fn set_interface(
    interface_name: String, siv: &mut Cursive, net_tx: &Sender<NetCommand>,
) {
    let result = net_tx.try_send(NetCommand::SetInterface {
        interface_name: interface_name.clone(),
    });

    if let Err(err) = result {
        ui::dialog::error::show_try_again(siv, err.to_string());
        return;
    }

    if let Ok(mut config) = CONFIG.try_lock() {
        config.interface_name = Some(interface_name);
        config.save().unwrap_or_default();
    }
}

pub fn set_language(language: String) {
    rust_i18n::set_locale(&language);

    if let Ok(mut config) = CONFIG.try_lock() {
        config.language = Some(language);
        config.save().unwrap_or_default();
    }
}

pub fn set_username(username: String, siv: &mut Cursive, net_tx: &Sender<NetCommand>) {
    let old_username = config::get_username();
    if old_username.eq(&username) {
        return;
    }

    let username = username
        .is_empty()
        .then(|| String::from("Anonymous"))
        .unwrap_or(username);

    let result = net_tx.try_send(NetCommand::UpdateUsername(username.clone()));

    if let Err(err) = result {
        ui::dialog::error::show_try_again(siv, err.to_string());
        return;
    }

    if let Ok(mut config) = CONFIG.try_lock() {
        config.username = Some(username.clone());
        config.save().unwrap_or_default();
    }

    ui::main_window::update_username_title(siv, &username);
}

pub fn show_message(
    id: ktp::Id, username: String, message: String, is_outgoing_message: bool,
    siv: &mut Cursive,
) {
    let now = chrono::offset::Local::now();

    let mut print = format!(
        "{time} [{username}] {message}",
        time = format!(
            "{hours:02}:{mins:02}:{secs:02}",
            hours = now.hour(),
            mins = now.minute(),
            secs = now.second()
        )
            // TODO: FIX THIS
        .dark_grey(),
        username = username.with(ui::colors::from_id(&id)),
    );

    if is_outgoing_message {
        print += &" sending...".dark_grey().to_string();
    }

    ui::cursive_extension::update_or_append_txt(siv, "chat_area", &message, print);
    if !is_outgoing_message {
        siv.call_on_name(&message, |child: &mut NamedView<TextView>| {
            child.set_name("");
        });
    }
}

pub fn presence_update(
    id: ktp::Id, username: String, is_inactive: bool, kind: UpdatePresenceKind,
    siv: &mut Cursive,
) {
    match kind {
        UpdatePresenceKind::JoinOrReconnect => {
            ui::cursive_extension::append_txt(
                siv,
                "chat_area",
                format!("> {username} logged on").dark_grey().to_string(),
            );
        },
        UpdatePresenceKind::UsernameChange { previous_username }
            if previous_username != username =>
        {
            ui::cursive_extension::append_txt(
                siv,
                "chat_area",
                format!("> {previous_username} is now known as {username}")
                    .dark_grey()
                    .to_string(),
            );
        },
        _ => {},
    }

    // Update username in presences list.
    ui::cursive_extension::update_or_append_txt(
        siv,
        "presences",
        &format!("{id:x?}_presence"),
        match is_inactive {
            true => format!("- {username}").dark_grey().to_string(),
            false => format!("{} {username}", "*".with(ui::colors::from_id(&id))),
        },
    );
}

pub fn remove_presence(id: ktp::Id, username: String, siv: &mut Cursive) {
    ui::cursive_extension::append_txt(
        siv,
        "chat_area",
        format!("> {username} disconnected, bye!")
            .dark_grey()
            .to_string(),
    );

    // Remove from presences list.
    siv.call_on_name("presences", |presences: &mut LinearLayout| {
        presences
            .find_child_from_name(&format!("{id:x?}_presence"))
            .map(|presence| presences.remove_child(presence));
    });
}
