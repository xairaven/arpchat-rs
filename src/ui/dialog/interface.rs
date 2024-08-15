use crate::net;
use crate::ui::commands;
use crossbeam::channel::Sender;
use cursive::views::SelectView;
use cursive::{traits::Resizable, views::Dialog, Cursive};

pub fn show_select_dialog(siv: &mut Cursive, ui_tx: Sender<commands::UI>) {
    let interfaces = net::interface::usable_sorted();

    siv.add_layer(
        Dialog::new()
            .title("Select an Interface")
            .content(
                SelectView::new()
                    .with_all(interfaces.into_iter().map(|interface| {
                        let name_id = match !&interface.description.is_empty() {
                            true => interface.description,
                            false => interface.name
                        };
                        let mac = interface.mac
                            .expect("All interfaces by contract must have MAC. Maybe, MAC dropped while menu was opening.");

                        const MAC_ALIGN: usize = 17;
                        const NAME_ALIGN: usize = 53;

                        (
                            format!("{:<NAME_ALIGN$}[{:>MAC_ALIGN$}]", name_id, mac),
                            name_id
                        )
                    }))
                    .on_submit(move |siv, interface_name_id: &String| {
                        let result = ui_tx.try_send(
                            commands::UI::SetInterface(
                                interface_name_id.to_owned()
                            ));

                        match result {
                            Ok(_) => {
                                siv.pop_layer();
                                // Next steps...
                            }
                            Err(err) => {
                                siv.add_layer(
                                    Dialog::text(err.to_string())
                                        .title("Error!")
                                        .button("Try again!", |siv| {
                                            siv.pop_layer();
                                        })
                                        .button("Quit", |siv| siv.quit()),
                                );
                            }
                        }
                    })
            )
            .button("Quit", |siv| siv.quit())
            .min_width(72)
    );
}
