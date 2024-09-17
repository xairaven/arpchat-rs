use crate::config::CONFIG;
use crate::net::commands::NetCommand;
use crate::ui::commands::UICommand;
use crate::ui::dialog;
use crate::{net, session, ui};
use crossbeam::channel::unbounded;
use cursive::Cursive;
use std::thread;

pub fn start() {
    let (ui_tx, ui_rx) = unbounded::<UICommand>();
    let (net_tx, net_rx) = unbounded::<NetCommand>();

    let mut ui_thread_username = String::from(session::INITIAL_USERNAME);

    log::info!("Channels created.");

    let net_thread = thread::Builder::new()
        .name("Net Thread".to_string())
        .spawn({
            let ui_tx = ui_tx.clone();
            move || net::core::start(ui_tx, net_rx)
        })
        .unwrap();
    log::info!("Net thread spawned.");

    let mut siv = cursive::default();
    siv.load_toml(include_str!("../../assets/styles.toml"))
        .expect("Styles are not loaded. Please, provide ./assets/styles.toml");

    log::info!("Styles loaded.");

    dialog::localization::show_select_dialog(&mut siv, ui_tx);

    let mut event_loop = siv.runner();
    event_loop.refresh();

    log::info!("Started event loop.");
    while event_loop.is_running() {
        while let Ok(command) = ui_rx.try_recv() {
            match command {
                UICommand::AlertUser => {
                    log::info!("UI Command: Alert User called.");
                    ui::commands::alert_user()
                },
                UICommand::SendNetError(err) => {
                    log::error!("Send Net Error: {}", err);
                    dialog::error::show_try_again(&mut event_loop, err);
                },
                UICommand::SendMessage { message_text } => {
                    log::info!("UI Command: Send message called.");
                    ui::commands::send_message(message_text, &mut event_loop, &net_tx)
                },
                UICommand::SetEtherType(ether_type) => {
                    log::info!("UI Command: Set EtherType called.");
                    ui::commands::set_ether_type(ether_type, &mut event_loop, &net_tx);
                },
                UICommand::SetInterface(interface_name) => {
                    log::info!("UI Command: Set Interface called.");
                    ui::commands::set_interface(interface_name, &mut event_loop, &net_tx);
                },
                UICommand::SetLanguage(language) => {
                    log::info!("UI Command: Set Language called.");
                    ui::commands::set_language(language);
                },
                UICommand::SetLogFileName(file_name) => {
                    log::info!("UI Command: Set Log File called.");
                    ui::commands::set_log_file_name(file_name);
                },
                UICommand::SetLogLevel(level) => {
                    log::info!("UI Command: Set Log Level called.");
                    ui::commands::set_log_level(level);
                },
                UICommand::SetUsername(username) => {
                    log::info!("UI Command: Set Username called.");
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
                    log::info!("UI Command: Show Message called.");
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
                    log::info!("UI Command: Presence Update called.");
                    ui::commands::presence_update(
                        id,
                        username,
                        is_inactive,
                        kind,
                        &mut event_loop,
                    );
                },
                UICommand::RemovePresence { id, username } => {
                    log::info!("UI Command: Remove Presence called.");
                    ui::commands::remove_presence(id, username, &mut event_loop);
                },
            }

            event_loop.refresh();
            log::trace!("Event loop refreshed.");
        }

        event_loop.step();
        log::trace!("Event loop step.");
    }

    net_tx.try_send(NetCommand::Terminate).unwrap();
    log::info!("Terminate command called.");

    net_thread.join().expect("Failed to join net thread");
}

pub fn quit(siv: &mut Cursive) {
    siv.quit();
    log::info!("UI Quit was called.");

    if let Ok(config) = CONFIG.try_lock() {
        config.save().unwrap_or_default();
        log::info!("Config saved while quitting.");
    }
}
