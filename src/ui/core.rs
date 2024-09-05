use crate::config::CONFIG;
use crate::net::commands::NetCommand;
use crate::ui::commands::UICommand;
use crate::ui::dialog;
use crate::{net, ui};
use crossbeam::channel::unbounded;
use cursive::Cursive;
use std::thread;

pub fn start() {
    let (ui_tx, ui_rx) = unbounded::<UICommand>();
    let (net_tx, net_rx) = unbounded::<NetCommand>();

    let net_thread = thread::spawn({
        let ui_tx = ui_tx.clone();
        move || net::core::start(ui_tx, net_rx)
    });

    let mut siv = cursive::default();
    siv.load_toml(include_str!("../../assets/styles.toml"))
        .expect("Styles are not loaded. Please, provide ./assets/styles.toml");

    dialog::localization::show_select_dialog(&mut siv, ui_tx);

    let mut event_loop = siv.runner();
    event_loop.refresh();

    while event_loop.is_running() {
        while let Ok(command) = ui_rx.try_recv() {
            match command {
                UICommand::AlertUser => ui::commands::alert_user(),
                UICommand::SendNetError(err) => {
                    dialog::error::show_try_again(&mut event_loop, err);
                },
                UICommand::SendMessage { message_text } => {
                    // TODO: Call net command
                },
                UICommand::SetEtherType(ether_type) => {
                    ui::commands::set_ether_type(ether_type, &mut event_loop, &net_tx);
                },
                UICommand::SetInterface(interface_name) => {
                    ui::commands::set_interface(interface_name, &mut event_loop, &net_tx);
                },
                UICommand::SetLanguage(language) => {
                    ui::commands::set_language(language);
                },
                UICommand::SetUsername(username) => {
                    ui::commands::set_username(username, &mut event_loop, &net_tx)
                },
                UICommand::ShowMessage {
                    id,
                    username,
                    message,
                } => {
                    // TODO: Show message
                },
            }

            event_loop.refresh();
        }

        event_loop.step();
    }

    net_tx.try_send(NetCommand::Terminate).unwrap();
    net_thread.join().expect("Failed to join net thread");
}

pub fn quit(siv: &mut Cursive) {
    siv.quit();

    if let Ok(config) = CONFIG.try_lock() {
        config.save().unwrap_or_default();
    }
}
