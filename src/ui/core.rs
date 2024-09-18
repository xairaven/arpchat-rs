use crate::config::CONFIG;
use crate::net::commands::NetCommand;
use crate::ui::commands::UICommand;
use crate::ui::dialog;
use crate::{net, session_settings, ui};
use crossbeam::channel::unbounded;
use cursive::Cursive;
use std::thread;

pub fn start() {
    let (ui_tx, ui_rx) = unbounded::<UICommand>();
    let (net_tx, net_rx) = unbounded::<NetCommand>();
    log::info!("Main UI and Net channels created.");

    let mut ui_thread_username = String::from(session_settings::INITIAL_USERNAME);

    let net_thread = thread::Builder::new()
        .name("Net Thread".to_string())
        .spawn({
            let ui_tx = ui_tx.clone();
            move || net::core::start(ui_tx, net_rx)
        })
        .unwrap_or_else(|err| {
            log::error!("Error: {err}");
            std::process::exit(1);
        });
    log::info!("Net thread spawned.");

    let mut siv = cursive::default();
    siv.load_toml(include_str!("../../assets/styles.toml"))
        .expect("Styles are not loaded. Please, provide ./assets/styles.toml");
    log::info!("Main styles from assets loaded.");

    dialog::localization::show_select_dialog(&mut siv, ui_tx);

    let mut event_loop = siv.runner();
    event_loop.refresh();

    log::info!("Started event loop.");
    while event_loop.is_running() {
        while let Ok(command) = ui_rx.try_recv() {
            match command {
                UICommand::AlertUser => {
                    log::info!("UI Command: Alert User.");
                    ui::commands::alert_user()
                },
                UICommand::ExportMessages(file) => {
                    log::info!("UI Command: Export Dialog.");
                    ui::commands::export_messages(&mut event_loop, file);
                },
                UICommand::SendNetError(err) => {
                    log::error!("UI Command: Net error. {}", err);
                    dialog::error::show_try_again(&mut event_loop, err);
                },
                UICommand::SendMessage { message_text } => {
                    log::info!("UI Command: Send message: {message_text}");
                    ui::commands::send_message(message_text, &mut event_loop, &net_tx)
                },
                UICommand::SetEtherType(ether_type) => {
                    log::info!("UI Command: Set EtherType: {ether_type}");
                    ui::commands::set_ether_type(ether_type, &mut event_loop, &net_tx);
                },
                UICommand::SetInterface(interface_name) => {
                    log::info!("UI Command: Set Interface: {interface_name}");
                    ui::commands::set_interface(interface_name, &mut event_loop, &net_tx);
                },
                UICommand::SetLanguage(language) => {
                    log::info!("UI Command: Set Language: {language}");
                    ui::commands::set_language(language);
                },
                UICommand::SetLogFileName(file_name) => {
                    log::info!("UI Command: Set Log File: {file_name}");
                    ui::commands::set_log_file_name(file_name);
                },
                UICommand::SetLogLevel(level) => {
                    log::info!("UI Command: Set Log Level: {level}");
                    ui::commands::set_log_level(level);
                },
                UICommand::SetUsername(username) => {
                    log::info!("UI Command: Set Username: {username}");
                    ui::commands::set_username(
                        username,
                        &mut ui_thread_username,
                        &mut event_loop,
                        &net_tx,
                    );
                },
                UICommand::ShowMessage {
                    id,
                    username,
                    message,
                    is_outgoing_message,
                } => {
                    log::info!("UI Command: Show Message: [{username}] {message}");
                    ui::commands::show_message(
                        id,
                        username,
                        message,
                        is_outgoing_message,
                        &mut event_loop,
                    );
                },
                UICommand::PresenceUpdate {
                    id,
                    username,
                    is_inactive,
                    kind,
                } => {
                    log::info!("UI Command: Presence Update. {username}: is inactive ({is_inactive})");
                    ui::commands::presence_update(
                        id,
                        username,
                        is_inactive,
                        kind,
                        &mut event_loop,
                    );
                },
                UICommand::RemovePresence { id, username } => {
                    log::info!("UI Command: Remove Presence: {username}");
                    ui::commands::remove_presence(id, username, &mut event_loop);
                },
            }

            event_loop.refresh();
        }

        event_loop.step();
    }

    net_tx
        .try_send(NetCommand::Terminate)
        .unwrap_or_else(|err| {
            log::error!("Error while Net.Terminate: {err}");
            quit(&mut siv);
        });

    net_thread.join().expect("Failed to join net thread");
}

pub fn quit(siv: &mut Cursive) {
    siv.quit();
    log::info!("UI Command: Quit.");

    if let Ok(config) = CONFIG.try_lock() {
        config.save().unwrap_or_default();
        log::info!("Config saved while quitting.");
    }
}
