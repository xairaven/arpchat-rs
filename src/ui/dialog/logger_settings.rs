use crate::config;
use crate::ui::commands::UICommand;
use crossbeam::channel::Sender;
use cursive::traits::Resizable;
use cursive::views::Dialog;
use cursive::Cursive;
use log::LevelFilter;

pub fn show_settings_dialog(siv: &mut Cursive, ui_tx: Sender<UICommand>) {
    let preferred_log_level: LevelFilter = config::lock_get_log_level();
    let preferred_log_filename: String = config::lock_get_log_filename();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.logging_settings"))
            .button(t!("button.close"), |siv| {
                siv.pop_layer();
            })
            .min_width(32)
            .max_width(56),
    );
}
