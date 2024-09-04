use crate::config::CONFIG;
use crate::ui::commands::UICommand;
use crate::{net, ui};
use crossbeam::channel::Sender;
use cursive::views::SelectView;
use cursive::{traits::Resizable, views::Dialog, Cursive};

pub fn show_select_dialog(siv: &mut Cursive, ui_tx: Sender<UICommand>) {
    const MAIN_WINDOW_INITIALIZED: bool = false;
    let interfaces = net::interface::usable_sorted();

    let preferred_interface_index = CONFIG
        .try_lock()
        .ok()
        .and_then(|locked_config| locked_config.interface_name.clone())
        .and_then(|config_interface_name| {
            interfaces
                .iter()
                .position(|interface| interface.name.eq(&config_interface_name))
        })
        .unwrap_or_default();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.interface_selection"))
            .content(
                SelectView::new()
                    .with_all(interfaces.into_iter().map(|interface| {
                        let name_id = match !&interface.description.is_empty() {
                            true => interface.description.to_owned(),
                            false => interface.name.to_owned(),
                        };
                        let mac = interface.mac.unwrap_or_default();

                        const MAC_ALIGN: usize = 17;
                        const NAME_ALIGN: usize = 53;

                        (
                            format!("{:<NAME_ALIGN$}[{:>MAC_ALIGN$}]", name_id, mac),
                            interface.name.to_owned(),
                        )
                    }))
                    .selected(preferred_interface_index)
                    .on_submit(move |siv, interface_name_id: &String| {
                        let result = ui_tx.try_send(UICommand::SetInterface(
                            interface_name_id.to_owned(),
                        ));

                        match result {
                            Ok(_) => {
                                siv.pop_layer();

                                ui::dialog::username::show_input_dialog(
                                    siv,
                                    ui_tx.clone(),
                                    MAIN_WINDOW_INITIALIZED,
                                )
                            },
                            Err(err) => {
                                ui::dialog::error::show_try_again(siv, err.to_string());
                            },
                        }
                    }),
            )
            .button(t!("button.quit"), ui::core::quit)
            .min_width(72),
    );
}
