use crate::config::CONFIG;
use crate::ui::commands::UI;
use crate::ui::{commands, dialog};
use crossbeam::channel::unbounded;

pub fn start() {
    let (ui_tx, ui_rx) = unbounded::<commands::UI>();
    let (net_tx, net_rx) = unbounded::<commands::Net>();

    let mut siv = cursive::default();

    dialog::localization::show_select_dialog(&mut siv, ui_tx);

    let mut event_loop = siv.runner();
    event_loop.refresh();

    while event_loop.is_running() {
        while let Ok(command) = ui_rx.try_recv() {
            match command {
                UI::SendMessage(_) => {},
                UI::SetInterface(_) => {},
                UI::SetLanguage(_) => {},
                UI::SetUsername(_) => {},
            }

            event_loop.refresh();
        }

        event_loop.step();
    }

    if let Ok(config) = CONFIG.try_lock() {
        config.save().unwrap_or_default();
    }
}
