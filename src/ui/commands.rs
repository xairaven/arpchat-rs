use crate::config::CONFIG;
use crate::error::net::NetError;
use crate::net::commands::NetCommand;
use crate::net::ether_type::EtherType;
use crate::{config, ui};
use crossbeam::channel::Sender;
use cursive::Cursive;

pub enum UICommand {
    NetError(NetError),

    SendMessage(String),

    SetEtherType(EtherType),
    SetInterface(String),
    SetLanguage(String),
    SetUsername(String),
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
    let result = net_tx.try_send(NetCommand::SetInterface(interface_name.clone()));

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
