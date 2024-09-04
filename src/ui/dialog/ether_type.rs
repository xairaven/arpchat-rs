use crate::config::CONFIG;
use crate::net::ether_type::EtherType;
use crate::ui;
use crate::ui::commands::UICommand;
use crossbeam::channel::Sender;
use cursive::traits::Resizable;
use cursive::views::{Dialog, LinearLayout, SelectView, TextView};
use cursive::Cursive;
use strum::IntoEnumIterator;

pub fn show_select_dialog(siv: &mut Cursive, ui_tx: Sender<UICommand>) {
    let preferred_ether_type_index = CONFIG
        .try_lock()
        .ok()
        .and_then(|locked_config| locked_config.ether_type)
        .and_then(|config_ether_type| {
            EtherType::iter().position(|ether_type| {
                ether_type.to_string().eq(&config_ether_type.to_string())
            })
        })
        .unwrap_or_default();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.protocol_selection"))
            .content(
                LinearLayout::vertical()
                    .child(TextView::new(t!("text.ether_types")))
                    .child(
                        SelectView::new()
                            .with_all(EtherType::iter().map(|ether_type| {
                                let name = &ether_type.to_string();

                                (name.to_string(), ether_type)
                            }))
                            .selected(preferred_ether_type_index)
                            .on_submit(move |siv, ether_type: &EtherType| {
                                let result = ui_tx.try_send(UICommand::SetEtherType(
                                    ether_type.to_owned(),
                                ));

                                match result {
                                    Ok(_) => {
                                        siv.pop_layer();
                                    },
                                    Err(err) => {
                                        ui::dialog::error::show_try_again(
                                            siv,
                                            err.to_string(),
                                        );
                                    },
                                }
                            }),
                    ),
            )
            .button(t!("button.close"), |siv| {
                siv.pop_layer();
            })
            .min_width(32)
            .max_width(56),
    );
}
