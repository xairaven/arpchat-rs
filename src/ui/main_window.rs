use crate::ui::commands::UICommand;
use crate::{config, ui};
use crossbeam::channel::Sender;
use cursive::event::Key;
use cursive::traits::{Nameable, Resizable, Scrollable};
use cursive::view::ScrollStrategy;
use cursive::views::{
    Dialog, EditView, LinearLayout, NamedView, Panel, ResizedView, ScrollView,
};
use cursive::{menu, Cursive};

pub const ELEMENT_NAME_CHAT_AREA: &str = "chat_area";
pub const ELEMENT_NAME_CHAT_INPUT: &str = "chat_input";
pub const ELEMENT_NAME_CHAT_PANEL: &str = "chat_panel";
pub const ELEMENT_NAME_ONLINE_PANEL: &str = "online_panel";

pub fn init(siv: &mut Cursive, ui_tx: Sender<UICommand>) {
    const AUTO_HIDE_MENU: bool = false;
    let initial_username = config::lock_get_username();

    siv.menubar()
        .add_subtree(
            t!("menu.settings"),
            menu::Tree::new()
                .leaf(t!("menu.change_username"), {
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
                .delimiter()
                .leaf(t!("menu.switch_protocol"), {
                    let ui_tx = ui_tx.clone();
                    move |siv| {
                        ui::dialog::ether_type::show_select_dialog(siv, ui_tx.clone());
                    }
                })
                .delimiter()
                .leaf(t!("menu.log_level"), {
                    let ui_tx = ui_tx.clone();
                    move |siv| {
                        ui::dialog::logger_settings::show_settings_log_level(
                            siv,
                            ui_tx.clone(),
                        );
                    }
                })
                .leaf(t!("menu.export_messages"), {
                    let ui_tx = ui_tx.clone();
                    move |siv| {
                        ui::dialog::export_messages::show_dialog(siv, ui_tx.clone());
                    }
                }),
        )
        .add_delimiter()
        .add_leaf(t!("menu.help"), show_help_dialog)
        .add_delimiter()
        .add_leaf(t!("menu.quit"), ui::core::quit);
    siv.set_autohide_menu(AUTO_HIDE_MENU);
    siv.add_global_callback(Key::Esc, |siv| siv.select_menubar());

    siv.add_fullscreen_layer(
        LinearLayout::horizontal()
            .child(
                LinearLayout::vertical()
                    .child(
                        Panel::new(
                            LinearLayout::vertical()
                                .with_name(ELEMENT_NAME_CHAT_AREA)
                                .full_height()
                                .full_width()
                                .scrollable()
                                .scroll_strategy(ScrollStrategy::StickToBottom),
                        )
                        .title(format!("arpchat: {initial_username}"))
                        .with_name(ELEMENT_NAME_CHAT_PANEL)
                        .full_height()
                        .full_width(),
                    )
                    .child(
                        Panel::new(
                            EditView::new()
                                .on_submit(move |siv, msg| {
                                    siv.call_on_name(
                                        ELEMENT_NAME_CHAT_INPUT,
                                        |input: &mut EditView| {
                                            input.set_content("");
                                        },
                                    );

                                    let result = ui_tx.try_send(UICommand::SendMessage {
                                        message_text: msg.to_string(),
                                    });

                                    if let Err(err) = result {
                                        ui::dialog::error::show_try_again(
                                            siv,
                                            err.to_string(),
                                        );
                                    }
                                })
                                .with_name(ELEMENT_NAME_CHAT_INPUT),
                        )
                        .full_width(),
                    )
                    .full_width(),
            )
            .child(
                Panel::new(
                    LinearLayout::vertical()
                        .with_name(ELEMENT_NAME_ONLINE_PANEL)
                        .full_height()
                        .full_width()
                        .scrollable()
                        .scroll_strategy(ScrollStrategy::StickToBottom),
                )
                .title(t!("title.online_users"))
                .full_height()
                .fixed_width(32),
            ),
    );

    show_help_dialog(siv);
}

pub fn update_username_title(siv: &mut Cursive, username: &str) {
    let title = format!("arpchat: {username}");
    type ChatPanel = Panel<ScrollView<ResizedView<ResizedView<NamedView<LinearLayout>>>>>;

    siv.set_window_title(&title);
    siv.call_on_name(ELEMENT_NAME_CHAT_PANEL, |chat_panel: &mut ChatPanel| {
        chat_panel.set_title(title);
    });
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
