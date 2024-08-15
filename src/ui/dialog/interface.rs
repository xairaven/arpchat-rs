use crate::ui::commands;
use crate::{net, ui};
use crossbeam::channel::Sender;
use cursive::views::SelectView;
use cursive::{traits::Resizable, views::Dialog, Cursive};

pub fn show_select_dialog(siv: &mut Cursive, ui_tx: Sender<commands::UI>) {
    let interfaces = net::interface::usable_sorted();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.interface_selection"))
            .content(
                SelectView::new()
                    .with_all(interfaces.into_iter().map(|interface| {
                        let name_id = match !&interface.description.is_empty() {
                            true => interface.description,
                            false => interface.name,
                        };
                        let mac =
                            interface.mac.expect(&t!("panic.mac_dropped"));

                        const MAC_ALIGN: usize = 17;
                        const NAME_ALIGN: usize = 53;

                        (
                            format!(
                                "{:<NAME_ALIGN$}[{:>MAC_ALIGN$}]",
                                name_id, mac
                            ),
                            name_id,
                        )
                    }))
                    .on_submit(move |siv, interface_name_id: &String| {
                        let result =
                            ui_tx.try_send(commands::UI::SetInterface(
                                interface_name_id.to_owned(),
                            ));

                        match result {
                            Ok(_) => {
                                siv.pop_layer();
                                ui::dialog::username::show_input_dialog(
                                    siv,
                                    ui_tx.clone(),
                                )
                            },
                            Err(err) => {
                                ui::dialog::error::show_try_again(
                                    siv,
                                    err.to_string(),
                                );
                            },
                        }
                    }),
            )
            .button(t!("button.quit"), |siv| siv.quit())
            .min_width(72),
    );
}
