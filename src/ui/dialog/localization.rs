use crate::ui;
use crate::ui::commands;
use crossbeam::channel::Sender;
use cursive::views::SelectView;
use cursive::{traits::Resizable, views::Dialog, Cursive};

pub fn show_select_dialog(siv: &mut Cursive, ui_tx: Sender<commands::UI>) {
    let locales = rust_i18n::available_locales!();

    siv.add_layer(
        Dialog::new()
            .title(t!("title.language_selection"))
            .content(
                SelectView::new()
                    .with_all(locales.into_iter().map(|language| {
                        (language.to_string(), language.to_owned())
                    }))
                    .on_submit(move |siv, language_name_id: &String| {
                        let result = ui_tx.try_send(commands::UI::SetLanguage(
                            language_name_id.to_string(),
                        ));
                        //////////////// TEMPORARY
                        rust_i18n::set_locale(language_name_id);
                        ////////////////

                        match result {
                            Ok(_) => {
                                siv.pop_layer();
                                ui::dialog::interface::show_select_dialog(
                                    siv,
                                    ui_tx.clone(),
                                );
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
            .button(t!("button.quit"), |siv| ui::core::quit(siv))
            .min_width(32),
    );
}
