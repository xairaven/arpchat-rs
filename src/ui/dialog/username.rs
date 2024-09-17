use crate::ui;
use crate::ui::commands::UICommand;
use crate::{config, session_settings};
use crossbeam::channel::{Sender, TrySendError};
use cursive::view::{Nameable, Resizable};
use cursive::views::{Dialog, EditView};
use cursive::Cursive;

pub fn show_input_dialog(
    siv: &mut Cursive, ui_tx: Sender<UICommand>, main_initialized: bool,
) {
    let dialog = Dialog::new()
        .title(t!("title.username_selection"))
        .content(
            EditView::new()
                .content(config::lock_get_username())
                .on_submit({
                    let ui_tx = ui_tx.clone();
                    move |siv, username| {
                        let username = session_settings::normalize_username(username);

                        let result =
                            ui_tx.try_send(UICommand::SetUsername(username.to_owned()));

                        process_operation_result(
                            siv,
                            main_initialized,
                            ui_tx.clone(),
                            result,
                        );
                    }
                })
                .max_content_width(session_settings::MAX_USERNAME_LENGTH)
                .with_name("username_input"),
        )
        .button(t!("button.save"), move |siv| {
            let username = siv
                .call_on_name("username_input", |input: &mut EditView| {
                    input.get_content()
                })
                .unwrap();
            let username = session_settings::normalize_username(username.as_str());

            let result = ui_tx.try_send(UICommand::SetUsername(username.to_string()));

            process_operation_result(siv, main_initialized, ui_tx.clone(), result);
        });

    // If window is initialized, "Close/quit button" will close dialog.
    // Otherwise, it will quit from an app. (Username set is required)
    let dialog = if main_initialized {
        dialog.button(t!("button.close"), |siv| {
            siv.pop_layer();
        })
    } else {
        dialog.button(t!("button.quit"), ui::core::quit)
    }
    .min_width(72);

    siv.add_layer(dialog);
}

fn process_operation_result(
    siv: &mut Cursive, main_initialized: bool, ui_tx: Sender<UICommand>,
    result: Result<(), TrySendError<UICommand>>,
) {
    match result {
        // If username set, initialize main window / close dialog
        Ok(_) => {
            siv.pop_layer();
            if !main_initialized {
                ui::main_window::init(siv, ui_tx.clone());
            }
        },
        // Otherwise, user have to try again
        Err(err) => {
            ui::dialog::error::show_try_again(siv, err.to_string());
        },
    }
}
