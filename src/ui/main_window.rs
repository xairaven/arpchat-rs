use crate::ui;
use crate::ui::commands;
use crossbeam::channel::Sender;
use cursive::event::Key;
use cursive::views::Dialog;
use cursive::Cursive;

pub fn init(siv: &mut Cursive, ui_tx: Sender<commands::UI>) {
    const AUTOHIDE_MENU: bool = false;
    show_help_dialog(siv);

    siv.menubar()
        .add_leaf(t!("menu.change_username"), {
            let ui_tx = ui_tx.clone();
            move |siv| {
                const MAIN_WINDOW_INITIALIZED: bool = true;
                ui::dialog::username::show_input_dialog(
                    siv,
                    ui_tx.clone(),
                    MAIN_WINDOW_INITIALIZED,
                );
            }
        })
        .add_delimiter()
        .add_leaf(t!("menu.help"), show_help_dialog)
        .add_delimiter()
        .add_leaf(t!("menu.quit"), |siv| siv.quit());
    siv.set_autohide_menu(AUTOHIDE_MENU);
    siv.add_global_callback(Key::Esc, |siv| siv.select_menubar());
}

fn show_help_dialog(siv: &mut Cursive) {
    siv.add_layer(
        Dialog::text(t!("text.help"))
            .title(t!("title.help"))
            .button(t!("button.ok"), |siv| {
                siv.pop_layer();
            }),
    );
}
