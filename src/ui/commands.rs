use crate::config::CONFIG;
use crate::net::commands::NetCommand;
use crate::{config, ui};
use crossbeam::channel::Sender;
use cursive::Cursive;

pub enum UICommand {
    SendMessage(String),

    SetInterface(String),
    SetLanguage(String),
    SetUsername(String),
}

pub fn set_username(
    username: String, siv: &mut Cursive, net_tx: &Sender<NetCommand>,
) {
    let old_username = config::get_username();
    if old_username.eq(&username) {
        return;
    }

    let username = username
        .is_empty()
        .then(|| String::from("Anonymous"))
        .unwrap_or(username);

    if let Ok(mut config) = CONFIG.try_lock() {
        config.username = Some(username.clone());
        config.save().unwrap_or_default();
    }

    let result = net_tx.try_send(NetCommand::UpdateUsername(username.clone()));

    if let Err(err) = result {
        ui::dialog::error::show_try_again(siv, err.to_string());
    }

    ui::main_window::update_username_title(siv, &username);
}
