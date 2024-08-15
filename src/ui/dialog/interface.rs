use crate::net;
use cursive::views::SelectView;
use cursive::{traits::Resizable, views::Dialog, Cursive};

pub fn show_interface_select_dialog(siv: &mut Cursive) {
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
                    .on_submit(|siv, interface_name_id: &String| {
                        siv.pop_layer();
                    })
            )
            .button("Quit", |siv| siv.quit())
            .min_width(72)
    );
}
