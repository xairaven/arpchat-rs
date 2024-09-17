use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::commands::NetCommand;
use crate::net::ether_type::EtherType;
use crate::net::ktp;
use crate::net::presence::UpdatePresenceKind;
use crate::{session, ui};
use chrono::Timelike;
use crossbeam::channel::Sender;
use cursive::backends::crossterm::crossterm::style::Stylize;
use cursive::utils::markup;
use cursive::views::{LinearLayout, NamedView, TextView};
use cursive::Cursive;
use log::LevelFilter;

pub enum UICommand {
    AlertUser,

    SendNetError(NetError),

    SendMessage {
        message_text: String,
    },

    SetEtherType(EtherType),
    SetInterface(String),
    SetLanguage(String),
    SetLogFileName(String),
    SetLogLevel(LevelFilter),
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
    // Ringing bell
    use std::io::{stdout, Write};
    print!("\x07");
    let _ = stdout().flush();
}

pub fn send_message(
    message_text: String, siv: &mut Cursive, net_tx: &Sender<NetCommand>,
) {
    if message_text.eq("/offline") {
        net_tx
            .try_send(NetCommand::PauseHeartbeat(true))
            .unwrap_or_else(|err| {
                log::error!("Error sending PauseHeartbeat with /offline: {}", err);
            });
    } else if message_text.eq("/online") {
        net_tx
            .try_send(NetCommand::PauseHeartbeat(false))
            .unwrap_or_else(|err| {
                log::error!("Error sending PauseHeartbeat with /online: {}", err);
            });
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

pub fn set_log_file_name(file_name: String) {
    if let Ok(mut config) = CONFIG.try_lock() {
        config.log_filename = Some(file_name);
        config.save().unwrap_or_default();
    }
}

pub fn set_log_level(level: LevelFilter) {
    if let Ok(mut config) = CONFIG.try_lock() {
        config.log_level = Some(level.to_string());
        config.save().unwrap_or_default();
    }
}

pub fn set_username(
    new_username: String, ui_username: &mut String, siv: &mut Cursive,
    net_tx: &Sender<NetCommand>,
) {
    if new_username.eq(ui_username) {
        return;
    }

    let username = session::normalize_username(&new_username);

    if let Ok(mut config) = CONFIG.try_lock() {
        config.username = Some(new_username.clone());
        config.save().unwrap_or_default();
    }

    let result = net_tx.try_send(NetCommand::UpdateUsername(username.clone()));

    if let Err(err) = result {
        ui::dialog::error::show_try_again(siv, err.to_string());
        return;
    }

    *ui_username = username.clone();
    ui::main_window::update_username_title(siv, &username);
}

pub fn show_message(
    id: ktp::Id, username: String, message: String, is_outgoing_message: bool,
    siv: &mut Cursive,
) {
    let now = chrono::offset::Local::now();
    let time = format!(
        "{hours:02}:{minutes:02}:{seconds:02}",
        hours = now.hour(),
        minutes = now.minute(),
        seconds = now.second()
    )
    .dark_grey();
    let username = username.with(ui::colors::from_id(&id));

    let mut print = format!("{time} [{username}] {message}");

    if is_outgoing_message {
        print += &t!("text.message_sending").dark_grey().to_string();
    }

    let print = markup::ansi::parse(print);

    ui::view_updater::update_or_append_txt(siv, "chat_area", &message, print);
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
            let translated = rust_i18n::replace_patterns(
                &t!("text.user_connected"),
                &["username"],
                &[username.clone()],
            );

            ui::view_updater::append_txt(
                siv,
                "chat_area",
                markup::ansi::parse(translated.dark_grey().to_string()),
            );
        },
        UpdatePresenceKind::UsernameChange { previous_username }
            if previous_username != username =>
        {
            let translated = rust_i18n::replace_patterns(
                &t!("text.user_changed_username"),
                &["username"],
                &[username.clone()],
            );
            let translated = rust_i18n::replace_patterns(
                &translated,
                &["previous_username"],
                &[previous_username],
            );

            ui::view_updater::append_txt(
                siv,
                "chat_area",
                markup::ansi::parse(translated.dark_grey().to_string()),
            );
        },
        _ => {},
    }

    // Update username in presences list.
    ui::view_updater::update_or_append_txt(
        siv,
        "online_panel",
        &format!("{id:x?}_presence"),
        match is_inactive {
            true => markup::ansi::parse(format!("- {username}").dark_grey().to_string()),
            false => markup::ansi::parse(format!(
                "{} {username}",
                "*".with(ui::colors::from_id(&id))
            )),
        },
    );
}

pub fn remove_presence(id: ktp::Id, username: String, siv: &mut Cursive) {
    let translated = rust_i18n::replace_patterns(
        &t!("text.user_disconnected"),
        &["username"],
        &[username.clone()],
    );

    ui::view_updater::append_txt(
        siv,
        "chat_area",
        markup::ansi::parse(translated.dark_grey().to_string()),
    );

    // Remove from presences list.
    siv.call_on_name("online_panel", |presences: &mut LinearLayout| {
        presences
            .find_child_from_name(&format!("{id:x?}_presence"))
            .map(|presence| presences.remove_child(presence));
    });
}
