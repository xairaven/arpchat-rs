use crate::config::CONFIG;
use crate::net::commands::NetCommand;
use crate::ui;
use crate::ui::commands::UICommand;
use crate::ui::dialog;
use crossbeam::channel::unbounded;
use cursive::Cursive;

pub fn start() {
    let (ui_tx, ui_rx) = unbounded::<UICommand>();
    let (net_tx, net_rx) = unbounded::<NetCommand>();

    let mut siv = cursive::default();
    siv.load_toml(include_str!("../../assets/styles.toml"))
        .expect("Styles are not loaded. Please, provide ./assets/styles.toml");

    dialog::localization::show_select_dialog(&mut siv, ui_tx);

    let mut event_loop = siv.runner();
    event_loop.refresh();

    while event_loop.is_running() {
        while let Ok(command) = ui_rx.try_recv() {
            match command {
                UICommand::SendMessage(_) => {},
                UICommand::SetInterface(_) => {},
                UICommand::SetLanguage(language) => {
                    ui::commands::set_language(language)
                },
                UICommand::SetUsername(username) => ui::commands::set_username(
                    username,
                    &mut event_loop,
                    &net_tx,
                ),
            }

            event_loop.refresh();
        }

        event_loop.step();
    }
}

pub fn quit(siv: &mut Cursive) {
    siv.quit();

    if let Ok(config) = CONFIG.try_lock() {
        config.save().unwrap_or_default();
    }
}
