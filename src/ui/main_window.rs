use crate::ui;
use crate::ui::commands;
use crossbeam::channel::{Sender, TrySendError};
use cursive::event::Key;
use cursive::traits::{Nameable, Resizable, Scrollable};
use cursive::view::ScrollStrategy;
use cursive::views::{Dialog, EditView, LinearLayout, Panel};
use cursive::Cursive;

pub fn init(siv: &mut Cursive, ui_tx: Sender<commands::UI>) {
    const AUTOHIDE_MENU: bool = false;

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

    siv.add_fullscreen_layer(
        LinearLayout::horizontal().child(
            LinearLayout::vertical()
                .child(
                    Panel::new(
                        LinearLayout::vertical()
                            .with_name("chat_area")
                            .full_height()
                            .full_width()
                            .scrollable()
                            .scroll_strategy(ScrollStrategy::StickToBottom),
                    )
                    .full_height()
                    .full_width(),
                )
                .child(
                    Panel::new(
                        EditView::new()
                            .on_submit(move |siv, msg| {
                                siv.call_on_name(
                                    "chat_input",
                                    |input: &mut EditView| {
                                        input.set_content("");
                                    },
                                );

                                let result = ui_tx.try_send(
                                    commands::UI::SendMessage(msg.to_string()),
                                );

                                if let Err(err) = result {
                                    ui::dialog::error::show_try_again(siv, err.to_string());
                                }
                            })
                            .with_name("chat_input"),
                    )
                    .full_width(),
                )
                .full_width(),
        ),
    );

    show_help_dialog(siv);
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
