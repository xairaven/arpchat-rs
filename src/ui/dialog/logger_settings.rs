use crate::ui::commands::UICommand;
use crate::{config, ui};
use crossbeam::channel::Sender;
use cursive::traits::Resizable;
use cursive::view::Nameable;
use cursive::views::{Dialog, LinearLayout, Panel, SelectView, TextView};
use cursive::Cursive;
use log::LevelFilter;

pub const ELEMENT_NAME_LOG_LEVEL_SELECTOR: &str = "settings_log_level_selector";

pub fn show_settings_log_level(siv: &mut Cursive, ui_tx: Sender<UICommand>) {
    let config_log_level_index = LevelFilter::iter()
        .position(|level| level.eq(&config::lock_get_log_level()))
        .unwrap_or_default();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.log_level"))
            .content(
                LinearLayout::vertical()
                    .child(Panel::new(
                        SelectView::new()
                            .with_all(LevelFilter::iter().map(|level| {
                                let name = &level.to_string();

                                (name.to_string(), level)
                            }))
                            .selected(config_log_level_index)
                            .on_submit({
                                let ui_tx = ui_tx.clone();
                                move |siv: &mut Cursive, level: &LevelFilter| {
                                    let result =
                                        ui_tx.try_send(UICommand::SetLogLevel(*level));
                                    match result {
                                        Ok(_) => {
                                            siv.pop_layer();
                                        },
                                        Err(err) => {
                                            ui::dialog::error::show_try_again(siv, err)
                                        },
                                    }
                                }
                            })
                            .with_name(ELEMENT_NAME_LOG_LEVEL_SELECTOR),
                    ))
                    .child(TextView::new(t!("text.changes_restart_needed"))),
            )
            .button(t!("button.save"), {
                let ui_tx = ui_tx.clone();
                move |siv| {
                    let index = siv
                        .call_on_name(
                            ELEMENT_NAME_LOG_LEVEL_SELECTOR,
                            |input: &mut SelectView| input.selected_id().unwrap(),
                        )
                        .unwrap();

                    let level = LevelFilter::iter()
                        .nth(index)
                        .unwrap_or(config::lock_get_log_level());
                    let result = ui_tx.try_send(UICommand::SetLogLevel(level));
                    match result {
                        Ok(_) => {
                            siv.pop_layer();
                        },
                        Err(err) => ui::dialog::error::show_try_again(siv, err),
                    }
                }
            })
            .button(t!("button.close"), |siv| {
                siv.pop_layer();
            })
            .min_width(32)
            .max_width(56),
    );
}
