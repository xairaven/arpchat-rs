use crate::ui;
use crate::ui::commands::UICommand;
use chrono::Datelike;
use crossbeam::channel::Sender;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};
use cursive::Cursive;
use std::fs::File;

pub const ELEMENT_NAME_EXPORT_CHAT_INPUT: &str = "export_chat_filename_input";

pub const FILE_NAME_TEMPLATE: &str = "EXPORTED_CHAT";

pub fn show_dialog(siv: &mut Cursive, ui_tx: Sender<UICommand>) {
    let now = chrono::offset::Local::now();
    let date = format!(
        "{year:04}-{month:02}-{day:02}",
        year = now.year(),
        month = now.month(),
        day = now.day(),
    );
    let possible_file_name = format!("{FILE_NAME_TEMPLATE}_{date}.txt");

    siv.add_layer(
        Dialog::new()
            .title(t!("title.export_messages"))
            .content(
                EditView::new()
                    .content(&possible_file_name)
                    .with_name(ELEMENT_NAME_EXPORT_CHAT_INPUT),
            )
            .button(t!("button.export"), move |siv| {
                let file_name = siv
                    .call_on_name(
                        ELEMENT_NAME_EXPORT_CHAT_INPUT,
                        |input: &mut EditView| input.get_content(),
                    )
                    .unwrap();

                let result = File::create(file_name.to_string());
                let file = match result {
                    Ok(value) => value,
                    Err(err) => {
                        ui::dialog::error::show_try_again(siv, err);
                        return;
                    },
                };

                if let Err(err) = ui_tx.try_send(UICommand::ExportMessages(file)) {
                    ui::dialog::error::show_try_again(siv, err);
                }
            })
            .button(t!("button.close"), |siv| {
                siv.pop_layer();
            })
            .min_width(56)
            .max_width(72),
    );
}
