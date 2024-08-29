use crate::ui;
use crate::ui::commands;
use crossbeam::channel::Sender;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

pub fn show_input_dialog(
    siv: &mut Cursive, ui_tx: Sender<commands::UI>, main_initialized: bool,
) {
    let dialog = Dialog::new()
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
                        let result = ui_tx.try_send(commands::UI::SetUsername(
                            username.to_owned(),
                        ));

                        match result {
                            Ok(_) => {
                                siv.pop_layer();
                                if !main_initialized {
                                    ui::main_window::init(siv, ui_tx.clone());
                                }
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
            let result =
                ui_tx.try_send(commands::UI::SetUsername(username.to_string()));

            match result {
                Ok(_) => {
                    siv.pop_layer();
                    if !main_initialized {
                        ui::main_window::init(siv, ui_tx.clone());
                    }
                },
                Err(err) => {
                    ui::dialog::error::show_try_again(siv, err.to_string());
                },
            }
        });

    // If window is initialized, "Close/quit button" will close dialog.
    // Otherwise, it will quit from an app. (Username set is required)
    let dialog = if main_initialized {
        dialog.button(t!("button.close"), |siv| {
            siv.pop_layer();
        })
    } else {
        dialog.button(t!("button.quit"), |siv| siv.quit())
    }
    .min_width(72);

    siv.add_layer(dialog);
}
