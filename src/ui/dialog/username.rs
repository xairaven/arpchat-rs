use crate::ui;
use crate::ui::commands;
use crossbeam::channel::Sender;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

pub fn show_input_dialog(siv: &mut Cursive, ui_tx: Sender<commands::UI>) {
    siv.add_layer(
        Dialog::new()
            .title(t!("title.username_selection"))
            .content(
                EditView::new()
                    .content(
                        gethostname::gethostname()
                            .to_string_lossy()
                            .split('.')
                            .next()
                            .unwrap_or("")
                            .to_string(),
                    )
                    .on_submit({
                        let ui_tx = ui_tx.clone();
                        move |siv, username| {
                            let result = ui_tx.try_send(
                                commands::UI::SetUsername(username.to_owned()),
                            );

                            match result {
                                Ok(_) => {
                                    siv.pop_layer();
                                    // Next steps...
                                },
                                Err(err) => {
                                    ui::dialog::error::show_try_again(
                                        siv,
                                        err.to_string(),
                                    );
                                },
                            }
                        }
                    })
                    .with_name("username_input"),
            )
            .button(t!("button.save"), move |siv| {
                let username = siv
                    .call_on_name("username_input", |input: &mut EditView| {
                        input.get_content()
                    })
                    .unwrap();
                let result = ui_tx
                    .try_send(commands::UI::SetUsername(username.to_string()));

                match result {
                    Ok(_) => {
                        siv.pop_layer();
                        // Next steps...
                    },
                    Err(err) => {
                        ui::dialog::error::show_try_again(siv, err.to_string());
                    },
                }
            })
            .button(t!("button.quit"), |siv| siv.quit())
            .min_width(72),
    );
}
